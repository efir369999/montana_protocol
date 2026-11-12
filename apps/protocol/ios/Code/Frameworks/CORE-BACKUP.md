# Code/Frameworks

The frameworks this application links are built locally and never committed:

- `MontanaBindings.xcframework`, the client core: `scripts/build-core.sh` with `MONTANA_CORE_SRC` set to
  `protocol/Code/crates/mt-bindings` of this repository;
- `MontanaCore.xcframework`, the reference implementation: `scripts/build-protocol-core.sh` with `MONTANA_PROTOCOL_CORE`
  set to `protocol/Montana-Protocol`;
- `WebRTC.xcframework`, the call engine: `fetch-webrtc.sh`, which pins the release by digest.
