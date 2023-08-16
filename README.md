# Simple Http Proxy
Just a simple http proxy that can sniff http(s) packets.

To allow tls sniffer you need to follow theses steps:
- install rootca.crt CA certificate located in src/tls/cert to your device
- now go to constants.rs
- change TLS value to **true** to allow connection decryption
- enable DEBUG 4° bit to sniff packets

note that sometimes TLS negotiation may fail because some clients verify server certificate without device CA trusted list.
