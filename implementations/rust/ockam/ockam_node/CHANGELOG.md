# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.10.0 - 2021-05-10
### Added
### Changed
- Context receive now uses a default or explicit timeout.
### Deleted

## v0.9.3 - 2021-05-03
### Changed
- Fix crate metadata.

## v0.9.2 - 2021-05-03
### Changed
- Updated dependencies.
- Lowered Context drop error log to trace.

## v0.9.1 - 2021-04-26
### Changed
- Updated dependencies.

## v0.9.0 - 2021-04-19
### Changed
- Fix return route while sending message.
- Updated dependencies.

## v0.8.0 - 2021-04-14
### Added
- Added an extra-tracing environment variable.
- Added dead_code lint.
- Added runtime getter to Node.
- Enabled multi-hop routes via domain specific routers.
- Gracefully handle an already initialised tracing system.

### Changed
- Use ockam_node `block_on` to avoid blocking tokio executor.

## v0.7.0 - 2021-04-13
### Changed
- Updated dependencies.
- Renamed Context address functions.

## v0.6.0 - 2021-04-12
### Added
- None to Node Error enum.
- Re-add block_future functionality.
- `send_message_from_address` added to Context.


### Changed
- Fixed panic when main Context goes out of scope.
- Refactored Node Context API.
- `msg_addr` moved from `Context` to `Routed`.
- Node won't spawn a worker with a with colliding address.

### Deleted
- Excess clones.

## v0.5.0 - 2021-04-05
### Added
- Expose onward route information to user workers.
- Make context receive retry receiving if wrong type.
- Handle message payloads without inner length.
- Make consuming cancel wrapper yield routed wrapper
- Use special message parsing for context receive.

### Changed
- Updated dependencies.
- Log worker errors instead of panicking.

### Deleted

## v0.4.0 - 2021-03-22
### Added

- Routing APIs.
- Router registration.
- Message forwarding.
- Mechanism for stopping a single worker.
- The `receive_match` API allows workers to block until receivng a specific message type.


### Changed

- Dependency updates.
- External router implementations don't need to accept TransportMessage, or parse specific user message types.
- Improved logging.

## v0.3.0 - 2021-03-04
### Added

- Worker mailbox queues.
- Support new Context API in `ockam_node_attribute`
- Message cancellation.
- Receive returns a result.

### Changed
- Refactor of `node` API.
- Node, Context and Worker APIs are now async.
- Worker initialization is now async.
- Dependency updates.
- Context `send_message` no longer silently eats errors.

## v0.2.0 - 2021-02-16

### Added
- Message trait implementation.
- Messages for starting and stopping Workers.

### Changed
- Changed internal registry implementation.

### Deleted
- Previous Message implementation (NodeMessage)
- Relay abstraction prototype

## v0.1.1 - 2021-02-04
### Added

- Added description to Cargo.toml.

## v0.1.0 - 2021-02-03
### Added

- This document and other meta-information documents.
