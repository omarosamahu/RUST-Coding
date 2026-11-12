UDS in Rust

An asynchronous implementation of Unified Diagnostic Services (UDS, ISO 14229) written in Rust with tokio. It receives diagnostic requests over a channel, dispatches them to the matching service, and returns a positive or negative response.

Status: work in progress. This is a learning project, and the service set is still growing.

Features
- Async request handling built on tokio (mpsc for requests, oneshot for per-request replies)
- Service dispatch by Service ID (SID) through a UDSService trait
- Positive responses and negative responses (0x7F <SID> <NRC>)

Supported services
SID	Service	Status
- 0x10	Diagnostic Session Control	**Implemented**
- 0x22	Read Data By Identifier	**Implemented**
- 0x2E	Write Data By Identifier	**Implemented**
- 0x31	Routine Control	**Planned**
- 0x3E	Tester Present	**Planned**
- 0x27	Security Access	**Planned**
