## mythical 0-link readable file and other wonkeries

here are some unassorted strangeness we encountered recently, while
using k8s/containers on linux.

# unbalanced directory index

one day, a k8s node logged these messages every few hours, in bundles of 4:

```
EXT4-fs warning (device md1): ext4_dx_add_entry:2380: Directory (ino: 211566352) index full, reach max htree level :2
EXT4-fs warning (device md1): ext4_dx_add_entry:2384: Large directory feature is not enabled on this filesystem
EXT4-fs warning (device md1): ext4_dx_add_entry:2380: Directory (ino: 211566352) index full, reach max htree level :2
EXT4-fs warning (device md1): ext4_dx_add_entry:2384: Large directory feature is not enabled on this filesystem
EXT4-fs warning (device md1): ext4_dx_add_entry:2380: Directory (ino: 211566352) index full, reach max htree level :2
EXT4-fs warning (device md1): ext4_dx_add_entry:2384: Large directory feature is not enabled on this filesystem
EXT4-fs warning (device md1): ext4_dx_add_entry:2380: Directory (ino: 211566352) index full, reach max htree level :2
EXT4-fs warning (device md1): ext4_dx_add_entry:2384: Large directory feature is not enabled on this filesystem
```

the mentioned directory was always inode `211566352`, so the situation
impacted just 1 workload. redhat has a [nice article](https://access.redhat.com/solutions/29894) explaining the
mechanism at work, and a note explaining why it happened occasionally:

> Files can still be successfully created if the hash of their filename does not fall into an already full hash bucket.

it includes an `auditd` log searching example at `syscall` level (eg
`creat`, `openat`, ...). however, at CocCoc, we're more familiar with
[`ebpf` tooling](https://github.com/iovisor/bpftrace/blob/master/docs/reference_guide.md#1-kprobekretprobe-dynamic-tracing-kernel-level),
and the message contained a function-ish hint
`ext4_dx_add_entry`. let's hunt for the failing process:

```bash
## is there some related probe/tracepoint?
$ sudo bpftrace -l '*ext4_dx*'
hardware:*ext4_dx*:
kprobe:ext4_dx_add_entry
kprobe:ext4_dx_csum
kprobe:ext4_dx_csum_set
kprobe:ext4_dx_find_entry
software:*ext4_dx*:


## yes, neat, let's grab any failling call & print out the process
$ sudo bpftrace -e 'kretprobe:ext4_dx_add_entry /retval !=0/ {printf("%d %s: %d\n", pid, comm, retval);}' |head -n 30
Attaching 1 probe...
2916472 php-fpm: -28
2916472 php-fpm: -28
2916472 php-fpm: -28
2916472 php-fpm: -28


## what is that
$ sudo systemctl status 2916472
...
kubepods-burstable.slice/kubepods-burstable-pod834b7b00_d731_4186_803c_8c4a35a84627.slice/cri-containerd-41b8343f32f8623c7d20942813b4a48c72cc0d34ae74
...
$ sudo crictl ps | grep 41b8343f3
...php...41b8343f3...some-app
```

from there, we could do `kubectl debug` into the container & look around for any huge directory. this app creates temporary files in a flat directory:

```
$ stat /app/tmp
  File: /app/tmp
  Size: 586358784       Blocks: 1145712    IO Block: 4096   directory
Device: 901h/2305d      Inode: 211566352   Links: 2


## that's our reported inode! what's in there? links=2 shows that there's no subdirectory
$ find /app/tmp -type f | wc -l
4781597
$ du -h /app/tmp
242G    /app/tmp
```

at this point, we made the mistake of rollout-restarting the deployment. new pods started working fine right away, but the old pods were still around in `Terminating` state. linux was busy removing the files in their overlayfs mount, including the above giant directories & their files. that caused ~3hours of high write-iops to commit filesystem metadata on the nodes, which slowed down other unrelated workloads. we had to cordon them during that time, and moved more important pods out of there.

the strangest effect was that pods on other nodes also failed readiness check randomly. it turned out some of their mysqlrouters were on the heavy-load nodes. the db clusters were totally fine, they run on different HW.

```
routing ERROR [7fc427ed7700] connecting to backend failed: Connection timed out (generic:110)
routing ERROR [7fc4286d8700] connecting to backend failed: not set (destinations:1)
routing ERROR [7fc425690700] connecting to backend failed: not set (destinations:1)
router Application got fatal signal: 11
router stack_bottom = 0 thread_stack 0x0
router /usr/lib64/mysqlrouter/private/libmysqlharness.so.1(my_print_stacktrace(unsigned char const*, unsigned long)+0x41) [0x7fc430782d71]
router /usr/lib64/mysqlrouter/private/libmysqlharness.so.1(+0xdc3b1) [0x7fc43076d3b1]
router /lib64/libpthread.so.0(+0x12d00) [0x7fc4303b4d00]
router /usr/lib64/mysqlrouter/private/libmysqlrouter_routing.so.1(MySQLRoutingAPI::get_destinations() const+0x22) [0x7fc42d04e4d2]
router /usr/lib64/mysqlrouter/private/libmysqlrouter_routing.so.1(+0x8e1bb) [0x7fc42d04f1bb]
```

i still don't understand how a network-heavy app can be disturbed so much by disk io. perhaps it checkpoints or logs something in critical path? but that's now [water under the bridge](https://youtu.be/4G-YQA_bsOU)

# filesystem IO-load and containerd

segue from the previous issue, for a long time we have been facing this situation:

- growing amount of linux process/thread (tasks) on some k8s nodes
- all of the extra ones are `containerd-shim` processes, with no
  related running containers

each shim is small, but the overall buildup causes system-wide degradation:

- `atop`, our favorite recording tool, starts failing with `Malloc failed for compression buffer`
- anything that scales per-process or per-cgroup has more useless work to do

here is an example which started running & finished its work during the above deletion:

```
## filtering out `collecting metrics` errors
$ sudo grep 513f9b55e96a /var/log/containerd.log.1|grep -v collecting|grep -o T.*
T14:14:36.809200001+07:00" level=info msg="CreateContainer within sandbox \"3ced0a60e406e5a4fff274f794947ff96129d15ccb8cabca3f3cc867511265da\" for &ContainerMetadata{Name:worker-5,Attempt:436,} returns container id \"513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f\""
T14:14:36.810770998+07:00" level=info msg="StartContainer for \"513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f\""
T14:14:37.979214517+07:00" level=info msg="StartContainer for \"513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f\" returns successfully"
T14:44:43.486606295+07:00" level=error msg="Failed to pipe stdout of container \"513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f\"" error="read /proc/self/fd/62: file already closed"
T14:44:52.439833260+07:00" level=error msg="failed to handle container TaskExit event &TaskExit{ContainerID:513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f,ID:513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f,Pid:211974,ExitStatus:0,ExitedAt:2023-09-06 07:44:42.439304235 +0000 UTC,XXX_unrecognized:[],}" error="failed to stop container: context deadline exceeded: unknown"
T14:47:02.214579151+07:00" level=info msg="TaskExit event &TaskExit{ContainerID:513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f,ID:513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f,Pid:211974,ExitStatus:0,ExitedAt:2023-09-06 07:44:42.439304235 +0000 UTC,XXX_unrecognized:[],}"
T14:52:34.752879642+07:00" level=info msg="RemoveContainer for \"513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f\""
T14:52:55.277595056+07:00" level=info msg="RemoveContainer for \"513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f\""
T14:52:55.277675179+07:00" level=error msg="RemoveContainer for \"513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f\" failed" error="failed to set removing state for container \"513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f\": container is already in removing state"
T14:53:00.815222826+07:00" level=info msg="RemoveContainer for \"513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f\" returns successfully"

## it's still here a day later, and will be forever more
$ ps aux|grep 513f9b55e96a
root      211874  0.0  0.0 708400  7324 ?        Sl   Sep06   0:02 containerd-shim -namespace k8s.io -workdir /var/lib/containerd/io.containerd.runtime.v1.linux/k8s.io/513f9b55e96a531360dc266d6c13124c5b7a2e027552245912203ab96b60e98f -address /run/containerd/containerd.sock -containerd-binary /usr/bin/containerd -systemd-cgroup
```

after 10s, from `ExitedAt` :44:42 to `deadline exceeded` :44:52, containerd gave up on removing the task, and the orphan shim stays around from then.

again with ebpf in one hand and the flow around this area in another, we think that discarding/unmounting each container's overlay filesystem are io-intensive as well as io-sensitive.

```bash
## with https://github.com/tanelpoder/0xtools
$ sudo psn -G kstack
 samples | avg_threads | comm                        | state                  | kstack
------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
       3 |        1.00 | (md*_resync)                | Disk (Uninterruptible) | kthread()->md_thread()->md_do_sync.cold.90()->msleep()
       2 |        0.67 | (containerd)                | Disk (Uninterruptible) | syscall_exit_to_user_mode()->exit_to_user_mode_prepare()->task_work_run()->cleanup_mnt()->deactivate_locked_super()->kill_anon_super()->generic_shutdown_super()->sync_filesystem()->ovl_sync_fs()->sync_filesystem()->__writeback_inodes_sb_nr()->wb_wait_for_completion()
       2 |        0.67 | (exe)                       | Disk (Uninterruptible) | __x64_sys_mount()->do_mount()->path_mount()->rwsem_down_write_slowpath()
```

```bash
## stack going back from sync_fs through to containerd process
# sorry, i forgot the command used to get this, perhaps something in bcc-tools for stack snooping
  ffffffffc067aea1 b'ext4_sync_fs'
  ffffffff818f55b1 b'sync_filesystem'
  ffffffffc07811e9 b'ovl_sync_fs'
  ffffffff818f55b1 b'sync_filesystem'
  ffffffff818bc9c2 b'generic_shutdown_super'
  ffffffff818bcab4 b'kill_anon_super'
  ffffffff818bc37f b'deactivate_locked_super'
  ffffffff818df498 b'cleanup_mnt'
  ffffffff816aa276 b'task_work_run'
  ffffffff8170fdf9 b'exit_to_user_mode_prepare'
  ffffffff81e882c8 b'syscall_exit_to_user_mode'
  ffffffff8200008c b'entry_SYSCALL_64_after_hwframe'
  55eb400e0750     b'syscall.Syscall'
  55eb40209055     b'github.com/containerd/containerd/mount.unmount'
  55eb4020930d     b'github.com/containerd/containerd/mount.UnmountAll'
  55eb4020d404     b'github.com/containerd/containerd/mount.WithTempMount.func2'
  55eb4020c7a3     b'github.com/containerd/containerd/mount.WithTempMount'
  55eb407b6860     b'github.com/containerd/containerd/oci.WithAdditionalGIDs.func1'
  55eb40e199d2     b'github.com/containerd/containerd/vendor/github.com/containerd/cri/pkg/containerd/opts.WithAdditionalGIDs.func1'
  55eb407af412     b'github.com/containerd/containerd/oci.ApplyOpts'
  55eb4080196a     b'github.com/containerd/containerd.WithSpec.func1'
  55eb407e37f9     b'github.com/containerd/containerd.(*Client).NewContainer'
  55eb40f06bb1     b'github.com/containerd/containerd/vendor/github.com/containerd/cri/pkg/server.(*criService).CreateContainer'
  55eb40f25abd     b'github.com/containerd/containerd/vendor/github.com/containerd/cri/pkg/server.(*instrumentedService).CreateContainer'
  55eb40da0278     b'github.com/containerd/containerd/vendor/k8s.io/cri-api/pkg/apis/runtime/v1alpha2._RuntimeService_CreateContainer_Handler.func1'
  55eb4106167f     b'github.com/containerd/containerd/vendor/github.com/grpc-ecosystem/go-grpc-prometheus.(*ServerMetrics).UnaryServerInterceptor.func1$
  55eb40d0bedd     b'github.com/containerd/containerd/vendor/k8s.io/cri-api/pkg/apis/runtime/v1alpha2._RuntimeService_CreateContainer_Handler'
  55eb40646676     b'github.com/containerd/containerd/vendor/google.golang.org/grpc.(*Server).processUnaryRPC'
  55eb4064a399     b'github.com/containerd/containerd/vendor/google.golang.org/grpc.(*Server).handleStream'
  55eb40657a1d     b'github.com/containerd/containerd/vendor/google.golang.org/grpc.(*Server).serveStreams.func1.1'
  55eb400c33e1     b'runtime.goexit'
    b'containerd' [1310]
```

cloudfoundry discussed [this general problem](https://www.cloudfoundry.org/blog/an-overlayfs-journey-with-the-garden-team/). coccoc is following with great interest for newer containerd 1.6 LTS releases with fixes around handling overlay deletion. some recent ones improved short-term, temporary mounts by marking them readonly. containerd maintainers also made a [great reproduction](https://github.com/containerd/containerd/pull/9004/files#diff-1d0d1c3863f35bb86ef37975c4e1a2062e6ca71e6f6a94dc385f8a3556284ddcR117) with `strace` fault-injection feature:

```bash
# step 2: strace to inject failpoint for the pod's shim
sudo strace -p $(pidof target-shim) --trace=umount2 -f --detach-on=execve -e inject=umount2:delay_enter=12s
```

a more fundamental fix, using overlayfs `volatile` mode to alleviate whole-system load, is [in design phase for now](https://github.com/containerd/containerd/pull/4785).

we can't do much else to mitigate this problem. due to the nature of php/nodejs/python applications with many loose files for each container, and the way we [pass php files to nginx containers](https://medium.com/coccoc-engineering-blog/our-journey-to-kubernetes-container-design-principles-is-your-back-bag-9166fc4736d2#957e) in a shared `emptyDir` volume.

# ghost hostPath files

onward to the main title. as part of migrating on-host applications to k8s, we mount some files as `hostPath` volume into containers, and let host cronjobs write new data into them. for a time, this worked correctly:

```yaml
## manifest fragments

volumes:
- name: host-data
  hostPath:
    path: /tmp/data.txt
    type: File
containers:
- name: app
  volumeMounts:
  - name: host-data
    mountPath: /data.txt
    readOnly: true
```

```bash
## updating from host is reflected in container
host$ echo version1 > /tmp/data.txt
app$ cat /data.txt
version1

host$ echo version2 > /tmp/data.txt
app $ cat /data.txt
version2
```

but update to the cronjob code introduced a new phenomenon. on host, we can see the new data in the file, but k8s pods read only old data.

```bash
host$ echo version3 > /tmp/new.txt
host$ mv -f /tmp/new.txt /tmp/data.txt
host$ cat /tmp/data.txt
version3

app $ cat /data.txt
version2
```

oops! here is the reason:

```bash
host$ stat /tmp/data.txt
  File: /tmp/data.txt
  Size: 9               Blocks: 8          IO Block: 4096   regular file
Device: 901h/2305d      Inode: 68812819    Links: 1

app $ stat /data.txt
  File: /data.txt
  Size: 9               Blocks: 8          IO Block: 4096   regular file
Device: 901h/2305d      Inode: 68812816    Links: 0
app $ mount|grep data
/dev/md1 on /data.txt type ext4 (ro,relatime,errors=remount-ro)
```

`hostPath` is implemented as a bind-mount, so it's "translated" to specific inode once at the pod setup phase. after `mv` rewrote the path to different inode, `68812816` is kept alive only by the mount namespace. it's similar to a running process holding open a deleted file, giving `DEL` state in `lsof` listings. but this 0-link file is still reachable from host, via the container's `root/` under `/proc`:

```bash
host$ sudo cat /proc/4185645/root/data.txt
version2
```

our mitigation for this one was changing `hostPath` up a level, to share the more-stable directory as volume instead. it would still break the same way if someone rename the directory, but it's less likely. and further, we'll work with people to share the data updates in a more robust way.

we hope you can have fun with the substrate under containers as we're having here.
