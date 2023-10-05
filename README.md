## ZKP Protocol
The following code utilizes ZKP Protocol - learn more in theory section of this repository.
The protocol is implemented as server and client using gRPC protocol according to the provided interface
described in “protobuf” schema. 

## Usage

### Local

Ensure that you have the following installed on your system
Rust:    https://www.rust-lang.org/
Protoc:  https://grpc.io/docs/protoc-installation/

Build:
```
cargo build
```

Terminal 1:
```
cargo run --bin server
```

Terminal 2:
```
cargo run --bin client
```

Behavior:

