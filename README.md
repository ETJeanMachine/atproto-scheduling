# ATProto Scheduling

A Rust-based web service for working with AT Protocol (ATProto) handles and OAuth flows.

## Current Features

- **Handle Resolution**: Resolves ATProto handles to DIDs using two methods:
  - Direct PDS lookup (when PDS is embedded in handle)
  - DNS TXT record lookup (`_atproto.<handle>`)
- **PDS Discovery**: Fetches Personal Data Server endpoints from DID documents
- **OAuth Framework**: Basic web server with OAuth routes (in progress)

## Supported Handle Types

- `did:plc` identities (via plc.directory)
- `did:web` identities (via .well-known/did.json)
- Handles with embedded PDS (e.g., `user.example.com`)
- DNS-configured handles (via TXT records)

## Running

```bash
cargo run
```

Server starts on `http://localhost:8080`

## Test URLs

- `http://localhost:8080/oauth?handle=jeanmachine.dev`
- `http://localhost:8080/oauth?handle=etjeanmachine.bsky.social`
- `http://localhost:8080/oauth?handle=nat.vg`

## License

WTFPL v2
