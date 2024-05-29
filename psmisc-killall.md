# psmisc 23.x killall doesn't match long process names

let's use more fool-proof methods

This is an [old][old], [fixed issue][fix] ([with more fixes to come][fixes]), but I would like to highlight it for anyone having trouble with killall. It affects Debian10 buster (stable release [from 2019 until 2021][buster-stable]), plus derivative Ubuntu versions. Reproduction based on Debian [bug #912748][bug]:

```dash
❯ podman run -it registry.hub.docker.com/amd64/debian:buster-slim bash -c 'apt update -qq && apt install -qqy psmisc procps; bash -i'

ln -sf /bin/sleep 0123456789abcd   && ./0123456789abcd   1h &
ln -sf /bin/sleep 0123456789abcde  && ./0123456789abcde  1h &
ln -sf /bin/sleep 0123456789abcdef && ./0123456789abcdef 1h &
```

With debian9, psmisc 22.21:

```dash
psmisc-22.21# pgrep -fla abcd
261 ./0123456789abcd 1h
264 ./0123456789abcde 1h
267 ./0123456789abcdef 1h

psmisc-22.21# killall -v -0 0123456789abcd
Killed 0123456789abcd(261) with signal 0
psmisc-22.21# killall -v -0 0123456789abcde
Killed 0123456789abcde(264) with signal 0
psmisc-22.21# killall -v -0 0123456789abcdef
Killed 0123456789abcdef(267) with signal 0
```

With debian10, psmisc 23.2:

```dash
psmisc-23.2# pgrep -fla abcd
322 ./0123456789abcd 1h
325 ./0123456789abcde 1h
328 ./0123456789abcdef 1h

psmisc-23.2# killall -v -0 0123456789abcd
Killed 0123456789abcd(322) with signal 0
psmisc-23.2# killall -v -0 0123456789abcde
Killed 0123456789abcde(325) with signal 0
Killed 0123456789abcde(328) with signal 0
psmisc-23.2# killall -v -0 0123456789abcdef
0123456789abcdef: no process found
```

With debian11, psmisc 23.4:

```dash
psmisc-23.4# killall -v -0 0123456789abcd
Killed 0123456789abcd(329) with signal 0
psmisc-23.4# killall -v -0 0123456789abcde
Killed 0123456789abcde(332) with signal 0
Killed 0123456789abcde(335) with signal 0
psmisc-23.4# killall -v -0 0123456789abcdef
Killed 0123456789abcde(332) with signal 0
Killed 0123456789abcde(335) with signal 0
```

You can imagine our surprise when a logrotate postrotate action didn’t poke any process, and the services keep sending log to a deleted file descriptor. Fortunately, we don’t use killall for other purposes, so the 15-character-prefix multikill didn’t bite us.

Our developers noticed missing log before we rolled this to production. They also took the initiative to convert convoluted init.d scripts to declarative systemd units. This avoids both name-based guessing like killall/pkill, and old-style PID-file based process management.

In the future, killall is looking into safer, non-pid-racing methods such as openat+pidfd. Let’s keep an eye out for such improvements, for situations where we can’t apply cgroups or other namespacing methods.

[old]: https://gitlab.com/psmisc/psmisc/-/commit/1188315cd037d73bf946a0003b70c6423cc330d2
[fix]: https://gitlab.com/psmisc/psmisc/-/commit/1188315cd037d73bf946a0003b70c6423cc330d2
[fixes]: https://gitlab.com/psmisc/psmisc/-/merge_requests/28
[buster-stable]: https://wiki.debian.org/DebianBuster
[pidfd]: https://gitlab.com/psmisc/psmisc/-/issues/37
[bug]: https://bugs.debian.org/912748
