Too many indirections and ASN749
===

Let's say your infrastructure uses CNAMEs to alias role to concrete nodes:

```zone
ejabberd24.example 30 IN CNAME vm1234
vm1234 30 IN A 10.00.12.34
```

You can connect to its 5222/tcp port as `ejabberd24.example:5222` as usual. Then you learned about SRV-based discovery, and want to let your users have nice handle instead of showing `ejabberd24` anywhere:

```zone
_xmpp-client._tcp.example 30 IN SRV 0 1 5222 ejabberd24.example
```

It works for profanity:

```sh
profanity --account user@example
```

Time changes, and the node grows more services: LDAP/kerberos, and a
Ceph monitor role.  Now your machines are port-knocking US DoD
graveyard netspace when they want to talk with ceph:

https://bgp.he.net/ip/6.118.109.49#_whois

```tcpdump
10.0.1.93	6.118.109.49	TCP	76	54342 → 3300 [SYN] Seq=0 Win=64240 Len=0 MSS=1460 SACK_PERM
```

What gives?

Hickory-DNS server reproduction
---

```toml
# named.toml for hickory-dns 0.24
log_level = "DEBUG"
listen_port = 4321
directory = "./srv-vs-cname/"

[[zones]]
  zone = "example"
  zone_type = "Primary"
  file = "example.zone"
```

```zone
; ./srv-vs-cname/example.zone
example. IN SOA localhost. localwriter.localhost. (
  7
  60
  60
  604800
  60
  )

_xmpp-client._tcp 30 IN SRV 10 10 5222 ejabberd24
_ceph-mon._tcp    30 IN SRV 10 20 3300 ejabberd24
_ldaps._tcp       30 IN SRV 10 30  636 ejabberd24

ejabberd24 30 IN CNAME vm1234
vm1234 30 IN A 10.0.12.34
```

Response looks good:

```text
❯ dig @127.0.0.1 -p 4321 -t srv _xmpp-client._tcp.example

;; QUESTION SECTION:
;_xmpp-client._tcp.example.     IN      SRV

;; ANSWER SECTION:
_xmpp-client._tcp.example. 30   IN      SRV     0 1 5222 ejabberd24.example.

;; ADDITIONAL SECTION:
ejabberd24.example.     30      IN      CNAME   vm1234.example.
vm1234.example.         30      IN      A       10.0.12.34
```

Insert our temporary server & domain into default interface, you can do it however you want

```bash
sudo-rs resolvectl dns wlp3s0 127.0.0.1:4321
sudo-rs resolvectl domain wlp3s0 example
```

It's more convenient:

```zone
❯ dig srv _ceph-mon._tcp.example

;; QUESTION SECTION:
;_ceph-mon._tcp.example.                IN      SRV

;; ANSWER SECTION:
_ceph-mon._tcp.example. 30      IN      SRV     10 20 3300 ejabberd24.example.

;; ADDITIONAL SECTION:
ejabberd24.example.     30      IN      CNAME   vm1234.example.
vm1234.example.         30      IN      A       10.0.12.34

;; Query time: 4 msec
;; SERVER: 127.0.0.53#53(127.0.0.53) (UDP)
```

```strace
❯ timeout 2s strace -fyy -e recvmsg,connect ceph-fuse -c /dev/null /none &| rg -v etc/ceph
strace: Process 163714 attached
strace: Process 163715 attached
[pid 163713] connect(3<UDP:[1684540]>, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr("127.0.0.53")}, 16) = 0
[pid 163713] connect(3<UDP:[1683641]>, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr("127.0.0.53")}, 16) = 0
[pid 163713] connect(3<UDP:[1682190]>, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr("127.0.0.53")}, 16) = 0
[pid 163713] connect(3<UDP:[1684542]>, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr("127.0.0.53")}, 16) = 0
strace: Process 163716 attached
strace: Process 163717 attached
strace: Process 163718 attached
strace: Process 163719 attached
strace: Process 163720 attached
strace: Process 163721 attached
[pid 163717] connect(12<TCP:[1682192]>, {sa_family=AF_INET, sin_port=htons(3300), sin_addr=inet_addr("6.118.109.49")}, 16) = -1 EINPROGRESS (Operation now in progress)
[pid 163717] connect(12<TCP:[10.0.6.193:58712->6.118.109.49:3300]>, {sa_family=AF_INET, sin_port=htons(3300), sin_addr=inet_addr("6.118.109.49")}, 16) = -1 EALREADY (Operation already in progress)
```

How does `6.118.109.49` appear in here?
