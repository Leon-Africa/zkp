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

### Docker Automation

Ensure that you have the follwing installed:

Docker:  https://www.docker.com/get-started/

Docker Compose: https://docs.docker.com/compose/

Build
```
docker-compose build
```

Start:
```
docker-compose up -d
```

Logs:
```
docker-compose logs zkp-server
```
```
docker-compose logs zkp-client
```

Container Status:
```
docker-compose ps
```

Interact with Client Container:
```
docker exec -it zkp-client sh

cargo run --bin client

