Too many indirections and AS749
===

Let's say your infrastructure uses CNAMEs to alias role to concrete nodes:

```zone
ejabberd24.example. 30 IN CNAME vm1234
vm1234 30 IN A 10.00.12.34
```

You can connect to its xmpp port as `ejabberd24.example:5222` as
usual. Then you learned about SRV-based discovery, and set this so
that your users have nice handles instead of showing `ejabberd24`
anywhere:

```zone
_xmpp-client._tcp.example 30 IN SRV 0 1 5222 ejabberd24.example.
```

It works for [profanity](https://xmpp.org/software/profanity/):

```sh
profanity --account user@example
```

Time changes, and the node grows more roles:
[LDAP/kerberos](https://ldap.com/dns-srv-records-for-ldap/), and a
[Ceph
monitor](https://docs.ceph.com/en/reef/rados/configuration/mon-lookup-dns/).
Now your machines are port-knocking [US DoD graveyard
netspace](https://bgp.he.net/ip/6.118.109.49#_whois) when they want to
talk with ceph:

```tcpdump
10.0.1.93	6.118.109.49	TCP	76	54342 → 3300 [SYN] Seq=0 Win=64240 Len=0 MSS=1460 SACK_PERM
```

What gives?

Hickory-DNS server reproduction
---

Here are some files to get a repro going:

```toml
# named.toml for hickory-dns 0.24 https://lib.rs/crates/hickory-dns/versions
listen_port = 4321
directory = "./"

[[zones]]
  zone = "example"
  zone_type = "Primary"
  file = "example.zone"
```

```zone
; ./example.zone
example. IN SOA localhost. localwriter.localhost. (
  7
  60
  60
  604800
  60
  )

_xmpp-client._tcp 30 IN SRV 10 10 5222 ejabberd24
_ceph-mon._tcp    30 IN SRV 10 20 3300 ejabberd24
_ldap._tcp        30 IN SRV 10 30  389 ejabberd24

ejabberd24 30 IN CNAME vm1234
vm1234     30 IN A    1.2.3.4
```

Response looks good:

```zone
❯ dig @127.0.0.1 -p 4321 -t srv _xmpp-client._tcp.example
;; QUESTION SECTION:
;_xmpp-client._tcp.example.     IN      SRV

;; ANSWER SECTION:
_xmpp-client._tcp.example. 30   IN      SRV     10 10 5222 ejabberd24.example.

;; ADDITIONAL SECTION:
ejabberd24.example.     30      IN      CNAME   vm1234.example.
vm1234.example.         30      IN      A       1.2.3.4

;; Query time: 0 msec
;; SERVER: 127.0.0.1#4321(127.0.0.1) (UDP)
```

Insert our temporary server & domain into default interface, you can
do it however your environment fits together:

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
vm1234.example.         30      IN      A       1.2.3.4

;; Query time: 4 msec
;; SERVER: 127.0.0.53#53(127.0.0.53) (UDP)
```

LDAP connects to the right address

```rust
❯ strace -yy ldapsearch -H 'ldap:///dc=example' &| rg connect.+389
connect(3<TCP:[1789186]>, {sa_family=AF_INET, sin_port=htons(389), sin_addr=inet_addr("1.2.3.4")}, 16)
```

Now on to ceph:

```rust
❯ timeout 2s strace -fyy -e connect ceph-fuse -c /dev/null /none &| rg -v 'etc/ceph|attached'
[pid 175097] connect(3<UDP:[1799840]>, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr("127.0.0.53")}, 16) = 0
[pid 175097] connect(3<UDP:[1802849]>, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr("127.0.0.53")}, 16) = 0
[pid 175097] connect(3<UDP:[1801817]>, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr("127.0.0.53")}, 16) = 0
[pid 175097] connect(3<UDP:[1801164]>, {sa_family=AF_INET, sin_port=htons(53), sin_addr=inet_addr("127.0.0.53")}, 16) = 0
[pid 175101] connect(12<TCP:[1801818]>, {sa_family=AF_INET, sin_port=htons(3300), sin_addr=inet_addr("6.118.109.49")}, 16) =
```

How does `6.118.109.49` appear in here?
