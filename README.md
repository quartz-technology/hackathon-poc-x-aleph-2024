# fs0x

fs0x - pronounced "f-s-x" - is a simple filesystem built using the Aleph acting as the storage layer.

💡 This project is built in the context of the Hackathon PoC x Aleph 2024.

## Table of Contents

- [Getting started](#getting-started)
  - [Requirements](#requirements)
  - [Run using Docker](#run-using-docker)
- [Contribute](#contribute)

## Getting started

### Requirements

- [Rust](https://www.rust-lang.org/): The perfect program for programs that interact with the Kernel
- [Docker](https://www.docker.com/): A container manager used to simplify the deployment of fs0x.

### Run using [Docker](https://www.docker.com/)

You will need two terminals opened in the container to test the project:
- One for the fx0x daemon.
- One for a basic shell terminal

1. Build the container

```shell
docker build -t fs0x .
```

2. Start the container

Creates a container named `fs0ox` that mounts your current directory into `/app`
and starts a shell in it.

```shell
docker run --name fs0x -it --privileged --rm -v $PWD:/app -w /app fs0x bash
```

> ⚠️ The container must be run using `--privileged` to by pass eventual issues
> happening on MacOS.

3. Open the client shell

In another terminal, start a shell in the same container

```shell
docker exec -it --privileged fs0x /bin/bash

# Create a temporary directory in your container
cd /tmp
mkdir test
```

4. Start the daemon

In the first terminal, start the daemon by mounting the filesystem you want to synchronize.

```shell
cargo run -- /tmp/test --id <my-id>  
```

> 💡 The `--id` option is the identifier for your session, please use the same on 
> the machine you want to synchronize with.

5. Create a peer

You can repeat the same process in another terminal or machine to set up a peer
that will synchronize the mounted filesystem.

## Contribute

This project is more than open for external contribution, feel free to open an issue 
for feature requests or bug reports.

If you want to directory contribute through a pull request, please read [CONTRIBUTING.md](./CONTRIBUTING.md).

Made with ❤️ by Quartz and [@martin-olivier](https://github.com/martin-olivier)