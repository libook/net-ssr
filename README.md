# net-ssr

[中文](./README.zh.md)

Network Secondary Surveillance Radar

Deploying transponders on a number of devices in the network allows you to quickly locate the IP address of those devices using an interrogator.

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
