# Changelog

All notable changes to this crate will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.12.0 - 2021-05-10
### Added
### Changed
- Renamed `async_worker` to `worker`.
### Deleted

## v0.11.2 - 2021-05-03
### Changed
- Updated dependencies.

## v0.11.1 - 2021-04-26
### Changed
- Updated dependencies.

## v0.11.0 - 2021-04-19
### Changed
- Updated dependencies.
- Updated a routing error internal domain code.

## v0.10.0 - 2021-04-14

### Changed
- Improved debug printability of addresses.
- Improved TCP transport initialisation.

## v0.9.0 - 2021-04-13
### Changed
- Updated dependencies.
- Renamed Context address functions.
- Improved printability of messages and payloads.

## v0.8.0 - 2021-04-12
### Added
- `Any` message type added to ockam_core crate.
### Changed
- Updated dependencies.
- `Routed` message wrapper function APIs renamed.
- `Passthrough` type renamed to `Any`.
- `msg_addr` moved from `Context` to `Routed`.
- `Context` address renamed to `primary_address`.
- Transport message fields renamed.


## v0.7.0 - 2021-04-05
### Added
- Expose onward route information to user workers.
- Random address generation.

### Changed
- Switch payload encoding from bincode to bare.
- Split forwarding examples into two binaries.
- Updated dependencies.
- Rename route fields in transport message.

### Removed
- RemoteMailbox has been moved to the `ockam` crate.


## v0.6.0 - 2021-03-22
### Added
- Routable message abstraction.
- Builder for Routable messags.
- Route metadata for messages.
- Generic transport message.

### Changed

- Updated dependencies.
- Core dependencies, such as hashbrown and hex have been re-exported from this crate.

## v0.5.0 - 2021-03-04
### Added

- Support multiple addresses per worker.

## v0.4.0 - 2021-03-03
### Added

- Auto-implementation of the `Message` trait for certain types.

### Changed

- The `Worker` trait and its methods are now async.
- Updated dependencies.

## v0.3.0 - 2021-02-16
### Added

- Explicit `alloc`, `no_std`, and `std` features.
- Generalized `Address` implementation.
- Global crate lib facade wrapper around `std` and `core` re-exports, for cross-feature compatibility.
- Message trait base implementation.

### Changed

- Updated dependencies.
- Improved documentation.



## v0.2.0 - 2021-02-04
### Added

-  Runs Worker `initialize` function when the Worker is started.
-  Uses `From` trait in place of `Into` in Node error
-  Create Worker example

### Removed
-  Worker Builder

### Changed

-  Moved Worker and Address types to this crate.
-  Renamed executor commands to messages

## v0.1.0 - 2021-01-30
### Added

- `Error` - an error type that can be returned is both `std` and `no_std` modes.
- `Result` - a result type that can be returned is both `std` and `no_std` modes.

