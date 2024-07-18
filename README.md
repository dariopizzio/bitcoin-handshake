# bitcoin-handshake

Bitcoin handshake POC in Rust

## Bitcoin handshake specification

<https://developer.bitcoin.org/reference/p2p_networking.html>

### Another useful resource

<https://learnmeabitcoin.com/technical/networking/#handshake>

## How to get a valid node IP address

You will find the information on this website
<https://bitnodes.io/nodes/?page=1>

## Improvements

- Improve socket logic
- Error handling
- Continue reading from the buffer after handshake (<https://learnmeabitcoin.com/technical/networking/#receiving-messages>)
- Add validation (checksum/protocol version/etc)
- Add Cli (and receive ip as parameter)
- Tests
- Improve Readme
