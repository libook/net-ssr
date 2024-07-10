# net-ssr

[中文](./README.zh.md)

Network Secondary Surveillance Radar

Deploying transponders on a number of devices in the network allows you to quickly locate the IP address of those devices using an interrogator.

## Installation

### Download from release

Go https://github.com/libook/net-ssr/releases and find your platform binary.

### Cargo install

```shell
cargo install net-ssr
```

### AUR

Go check https://aur.archlinux.org/packages/net-ssr
Use any AUR helper to install. For example:
```shell
paru -S net-ssr
yay -S net-ssr
```

## Docker/Podman

```shell
# Run transponder in the background
# via Docker:
docker run -it -d --network=host --name transponder libook/net-ssr-transponder
# via Podman:
podman run -it -d --network=host --name transponder libook/net-ssr-transponder

# Run interrogator for one time
# via Docker:
docker run -it --rm --network=host --name interrogator libook/net-ssr-interrogator -v
# via Podman:
podman run -it --rm --network=host --name interrogator libook/net-ssr-interrogator -v
```

### Compile from source
```shell
git clone https://github.com/libook/net-ssr.git
cd net-ssr
cargo build --release
cd target/release
```

## Usage

On devices that are required to report IP, ensure that the transponder is running:
```shell
transponder
```
This will start a process listening on port 1030. When an interrogator asks within the network, the transponder will answer the IP address.

On a device that needs to find answering devices, run the interrogator:
```shell
interrogator
```
This will broadcast an interrogation into the network(defaults to all networks currently accessed by the device) and listen on port 1090. When an answering machine answers with an IP address, the interrogator will print out the IP address.
The interrogator will continue to wait for answers until the user exits using `Ctrl+c`.

There are a variety of parameters that can be configured, check the help using `-h`.
```shell
transponder -h
interrogator -h
```

## About the name

net-ssr is short for Network Secondary Radar, and its naming is inspired by the Secondary Surveillance Radar in the aviation field.
In the secondary radar system, the aircraft are equipped with automatic transponder, ground stations or other aircraft can use the interrogator to send interrogation, the transponder receives the interrogation, will automatically return the aircraft code, altitude and other information.
