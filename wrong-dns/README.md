# Have you seen DNS TYPE0 CLASS256?

we run a recursive resolver for historical reason. these recent months, we started seeing messages like this in unbound.log, with different names:

```text
info: resolving cctv-api.cdn20.com.cd23f.com. TYPE0 CLASS256
info: resolving cctv-api.cdn20.com.chnc.cloudcsp.com. TYPE0 CLASS256
info: resolving cctv-api.wswebpic.com.chnc.cloudcsp.com. TYPE0 CLASS256
info: resolving overseatest.livehttp.speedcdns.com. TYPE0 CLASS256
info: resolving wangsu-1.ttyuyin.com.cdn20.com. TYPE0 CLASS256
info: resolving wangsu-3.ttyuyin.com.cdn20.com. TYPE0 CLASS256
```

afaik we have no relationship with such infrastructure (CCTV in some Chinese cloud?). we blocked the most queried labels, and move on to other things. however i have a nagging question: what is that?

i still have no details about the chinese parties, but after some searching & reproduction, i think there is a defect-attractor in DNS-packet construction. in order to avoid search-suffix behaviour, people write `.`-ending names in config, and some software will construct an invalid DNS query:

- haproxy, before 2020 https://github.com/haproxy/haproxy/issues/435
- gnupg dirmngr, in 2017 https://dev.gnupg.org/T3517#106524
- nongnu LWIP, in 2020 https://savannah.nongnu.org/bugs/?58473#comment3

the sequence is potentially like this:

1. split name by `.`.
1. for each item, construct `${length}${label}` segments. the final empty `label` gets converted to its own `0x0` byte.
1. concat the segments together, then append the empty label `0x0`, regardless of previous step.
1. append the DNS type & class (always IN/`0x1` for Internet)

Thus, now we have a name that ends with `\0\0` (or `00 00` in hexdump view). The question now appends usual Type A (`00 01`), Class IN (`00 01`), but those become misaligned:

```text
Okay:
03 ?? ?? ?? 00|  00 01|    00 01
ll  c  o  m . | TYPE A|CLASS  IN

Wrong:
03 ?? ?? ?? 00|  00 00|    01 00 01
ll  c  o  m . | TYPE 0|CLASS 256 FORMAT ERROR, i hope the server answer
```

for more background, check out [jvns' blog](https://jvns.ca/blog/2022/09/12/why-do-domain-names-end-with-a-dot-/#in-a-dns-request-response-domain-names-don-t-have-a-trailing) or view responses in various formats with [DNS institute tool](https://dnsinstitute.com/webdnsquery/).

there is a [different case from 2001](https://groups.google.com/g/comp.protocols.dns.bind/c/pIVLil7p8wA/m/Y0bxMyn5bI0J), where a (windows-based?) client sent TYPE & CLASS as little-endian, not big-endian, resulting in `TYPE256 CLASS256` query.

## Reproduction

> a single ping, comrade. just one, for range. -- Marko Ramius

catching in wireshark/tshark:

`dns.qry.class > 1`

or with tcpdump/pcap filter:

`udp port 53 and dns.query.class > 1`

i want to generate those wrong Questions on-demand, to observe and quote the various DNS servers' log lines about them, as well as to put [all kind of TYPEs](https://www.netmeister.org/blog/dns-rrs.html) in the Q and see what they get intepreted as.

```bash
$ declare -a TYPES=(type1 type2 cname type6 mx txt aaaa srv dname type65 type255 caa)
```

The core code is:

```rust
fn naughty_question(qtype: Rtype) -> Question<Dname<Vec<u8>>> {
    let name = Dname::vec_from_str("example.").unwrap();
    let mut bytes = name.into_octets();
    bytes.push(0);
    Question::new_in(
        // Safety: I want to do it wrong
        unsafe { Dname::from_octets_unchecked(bytes) },
        qtype,
    )
}
#[test]
fn it_is_different() {
    assert!(
        !Question::from((Dname::vec_from_str("example.").unwrap(), Rtype::Any))
            .eq(&naughty_question(Rtype::Any))
    );
}
```

i chose NLnet Labs' `domain` crate, because it is self-contained, comes
with a resolver. fortunately it exposes the above hand-off function to give users' direct access to parsed domain bytes. i haven't wrapped my head around how to properly print out all kinds of answer data yet, so i resigned to asserting that the queried server ignores/refuses the bad questions. i can construct a bad `Dname`, pinky-promise to `domain` that it is good, and let it help me with the rest of DNS+UDP wrapping & handling!

an alternative would be `trust-dns`'s `proto` & `client` crates. i don't think its server-side implementation is ready yet.

I queried some usual resource types against an unbound server:

```bash
$ declare -a TYPES=(type1 type2 cname type6 mx txt aaaa srv dname type65 type255 caa)
$ for type in ${TYPES[*]}; do cargo r -q -- @unbound.address $type; done

A: b"\0\0\x81\x85\0\x01\0\0\0\0\0\0\x07example\0\0\0\x01\0"
NS: server ignored us, perhaps: Custom { kind: TimedOut, error: "all timed out" }
CNAME: b"\0\0\x81\x85\0\x01\0\0\0\0\0\0\x07example\0\0\0\x05\0"
SOA: b"\0\0\x81\x85\0\x01\0\0\0\0\0\0\x07example\0\0\0\x06\0"
MX: b"\0\0\x81\x85\0\x01\0\0\0\0\0\0\x07example\0\0\0\x0f\0"
TXT: b"\0\0\x81\x85\0\x01\0\0\0\0\0\0\x07example\0\0\0\x10\0"
AAAA: b"\0\0\x81\x85\0\x01\0\0\0\0\0\0\x07example\0\0\0\x1c\0"
SRV: b"\0\0\x81\x85\0\x01\0\0\0\0\0\0\x07example\0\0\0!\0"
DNAME: server ignored us, perhaps: Custom { kind: TimedOut, error: "all timed out" }
HTTPS: server ignored us, perhaps: Custom { kind: TimedOut, error: "all timed out" }
ANY: b"\0\0\x81\x85\0\x01\0\0\0\0\0\0\x07example\0\0\0\xff\0"
CAA: server ignored us, perhaps: Custom { kind: TimedOut, error: "all timed out" }
```

"perhaps" is because, as we see below, the program sometimes doesn't receive response data. unbound logs clear messages:

```text
unbound[1626:0] info: resolving example. TYPE0 CLASS256
unbound[1626:4] info: resolving example. TYPE0 CLASS512
unbound[1626:2] info: resolving example. TYPE0 CLASS1280
unbound[1626:3] info: resolving example. TYPE0 CLASS1536
unbound[1626:1] info: resolving example. TYPE0 CLASS3840
unbound[1626:5] info: resolving example. TYPE0 CLASS4096
unbound[1626:2] info: resolving example. TYPE0 CLASS7168
unbound[1626:4] info: resolving example. TYPE0 CLASS8448
unbound[1626:3] info: resolving example. TYPE0 CLASS9984
unbound[1626:3] info: resolving example. TYPE0 CLASS65280
```

Here is tcpdump's translation of the traffic:

```text
0+ [1au] Type0 (Class 256)? example. (37)
0 FormErr 0/0/1 (36)
0+ Type0 (Class 256)? example. (26)
0 Refused 0/0/0 (25)
0+ [1au] Type0 (Class 512)? example. (37)
0 FormErr 0/0/1 (36)
0+ Type0 (Class 512)? example. (26)
0 Refused 0/0/0 (25)
0+ [1au] Type0 (Class 1280)? example. (37)
0 FormErr 0/0/1 (36)
0+ Type0 (Class 1280)? example. (26)
0 Refused 0/0/0 (25)
0+ [1au] Type0 (Class 1536)? example. (37)
0 FormErr 0/0/1 (36)
0+ Type0 (Class 1536)? example. (26)
0 Refused 0/0/0 (25)
0+ [1au] Type0 (Class 3840)? example. (37)
0 FormErr 0/0/1 (36)
0+ Type0 (Class 3840)? example. (26)
0 Refused 0/0/0 (25)
0+ [1au] Type0 (Class 4096)? example. (37)
0 FormErr 0/0/1 (36)
0+ Type0 (Class 4096)? example. (26)
0 Refused 0/0/0 (25)
0+ [1au] Type0 (Class 7168)? example. (37)
0 FormErr 0/0/1 (36)
0+ Type0 (Class 7168)? example. (26)
0 Refused 0/0/0 (25)
0+ [1au] Type0 (Class 8448)? example. (37)
0 FormErr 0/0/1 (36)
0+ Type0 (Class 8448)? example. (26)
0 Refused 0/0/0 (25)
0+ [1au] Type0 (Class 9984)? example. (37)
0 FormErr 0/0/1 (36)
0+ Type0 (Class 9984)? example. (26)
0 Refused 0/0/0 (25)
0+ [1au] Type0 (Class 16640)? example. (37)
0 FormErr 0/0/1 (36)
0+ Type0 (Class 16640)? example. (26)
0 Refused 0/0/0 (25)
0+ [1au] Type0 (Class 65280)? example. (37)
0 FormErr 0/0/1 (36)
0+ Type0 (Class 65280)? example. (26)
0 Refused 0/0/0 (25)
0+ [1au] A (Class 256)? example. (37)
0 FormErr 0/0/1 (36)
```

(you see why i said `perhaps`). this last one `A (Class 256)?` is from a `CAA` rtype. its high byte gets reinterpreted as `type A` again! but the `Class 256` is never fixable, and the whole packet is malformed beyond repair anyhow.

i hope someone out there come across this & figure out whatever is happening with the clients sending these queries.

i tested again ISC bind, but it says only:

```text
client: warning: client 4.5.6.7#33870: message parsing failed: unexpected end of input
```

CoreDNS, NLnet NSD, systemd-resolved doesn't log anything about these at all in my testing.

of the cloudflare addresses, .1 and .3 replies quickly & i can print the response, but i miss .2 traffic consistently:

```bash
$ for s in 1 2 3; do cargo r -q -- @1.1.1.$s aaaa; done
AAAA: b"\0\0\x81\x84\0\x01\0\0\0\0\0\0\x07example\0\0\0\x1c\0"
AAAA: server ignored us, perhaps: Custom { kind: TimedOut, error: "all timed out" }
AAAA: b"\0\0\x81\x84\0\x01\0\0\0\0\0\0\x07example\0\0\0\x1c\0"
```

nothing comes back from .2:

```text
0+ [1au] Type0 (Class 7168)? example. (37)
0 NotImp 0/0/0 (25)
0+ [1au] Type0 (Class 7168)? example. (37)
0+ [1au] Type0 (Class 7168)? example. (37)
0 NotImp 0/0/0 (25)
```
