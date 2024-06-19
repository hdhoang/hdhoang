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

On the wire
---

Let's capture some packets with tcpdump/wireshark for
`dns.srv.port==3300 or tcp.port==3300`. Here is the DNS answer's
additional records section

```hexdump
0000   0a 65 6a 61 62 62 65 72 64 32 34 c0 1b 00 05 00   .ejabberd24.....
0010   01 00 00 00 1e 00 09 06 76 6d 31 32 33 34 c0 1b   ........vm1234..
0020   c0 65 00 01 00 01 00 00 00 1e 00 04 01 02 03 04   .e..............
0030   00 00 29 ff d6 00 00 00 00 00 00                  ..)........
```

With
[name-compression](https://dotat.at/@/2022-07-01-dns-compress.html),
`.example.` was back-referenced to earlier data, hence we see only the
`ejabberd24` and `vm1234` as text, preceeded by the own lengths (0x0a
= 10, 0x06 = 6). `ceph-fuse` looked for 32 bits of ipv4 address in
there, interpreted `06 76 6d 31` in [decimal
format](https://manned.org/ascii.7) as `06.118.109.49`, and
[mis-dialed
DOD](https://scribe.rip/have-you-seen-dns-type0-class256-896b10af92fc).

Why would it do that when the rrtype is `CNAME` (`0005`)? Why doesn't
it assume ipv6 and scraped out 16 bytes as
`[0676:6d31:3233:34c0:1bc0:6500:0100:0100]`? I have no desire to look
into current implementation.

Yet, is it in the right? Yes, [RFC
7287](https://datatracker.ietf.org/doc/html/rfc2782) page 4 requires
that

> the name MUST NOT be an alias

Or as [cloudflare
illustrates](https://www.cloudflare.com/learning/dns/dns-records/dns-srv-record/):

> SRV records must point to an A record (in IPv4) or an AAAA record
> (in IPv6). The server name they list cannot be a CNAME. So
> "server.example.com" must lead directly to an A or AAAA record under
> that name.

The correct zone data must be:

```zone
_xmpp-client._tcp 30 IN SRV 10 10 5222 vm1234 ; current node for ejabberd24
_ceph-mon._tcp    30 IN SRV 10 20 3300 vm1234 ; current node for ceph-mon
_ldap._tcp        30 IN SRV 10 30  389 vm1234 ; current node for openldap
```

And we will curse ourselves for not updating comments down the years.
