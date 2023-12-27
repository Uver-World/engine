# Uver-World Client


## Dependencies

- [build-essential](https://packages.debian.org/bookworm/build-essential)
- [pkg-config](https://packages.debian.org/bookworm/pkg-config)
- [libasound2-dev](https://packages.debian.org/bookworm/libasound2-dev)
- [libssl-dev](https://packages.debian.org/bookworm/libssl-dev)
- [libudev-dev](https://packages.debian.org/bookworm/libudev-dev)
- [rust](https://sh.rustup.rs)

### Running in online mode
> The online mode requires the following
> - REST API
> - WebRTC signaling server
- Just run `cargo run [profile]`

### Running in offline mode
- Run `cargo run -- -o [profile]`

## SigNoz
- https://signoz.io/docs/install/docker/
