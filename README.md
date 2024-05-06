[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
# Netwerk

netwerk is a wip p2p network using fec via raptorq over udp

## Overview

## Todo

- [x] Implement server and client, verify they communicate with each other
- [x] Implement raptorq for the UDP connection to be forward error correcting
- [ ] write tests to verify the raptorq implementation and the UDP connection latency, packet loss, and bandwidth
- [ ] extend communication to be able to send messages (beyond just the handshake and ping)
- [ ] Implement a way to send messages to all clients
- [ ] Implement a way to send messages to a specific client
- [ ] Design a control protocol for the server to manage the clients
- [ ] Implement hole punching
- [ ] Implement peer2peer communication

## Implementation

https://www.qualcomm.com/media/documents/files/raptorq-technical-overview.pdf

https://docs.rs/libp2p/latest/libp2p/tutorials/hole_punching/index.html

https://github.com/libp2p/specs/blob/master/rendezvous/
