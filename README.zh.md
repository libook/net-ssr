# net-ssr

[English](./README.md)

网络二次雷达系统

在网络中的若干设备上部署应答机（transponder），即可使用询问机（interrogator）快速定位到这些设备的IP地址。

## 用法

在需要报告IP的设备上，确保应答机处于运行状态:
```shell
transponder
```
这将启动一个进程监听1030端口。当有询问机在网络内询问时，应答机将回答IP地址。

在需要查找应答设备的设备上，运行询问机:
```shell
interrogator
```
这将向网络（默认为当前设备接入的所有网络）内广播询问，并监听1090端口。当有应答机回答IP地址时，询问机将打印出IP地址。
询问机将持续等待应答，直到用户使用`Ctrl+c`退出。

有多种参数可以配置，可以使用`-h`查看帮助。
```shell
transponder -h
interrogator -h
```

## 关于名称

net-ssr是网络二次雷达的简称，其命名灵感来源于航空领域的二次雷达（Secondary Surveillance Radar）。
在二次雷达系统中，飞行器都装配有自动应答机（Transponder），地面站或其他飞行器可以使用询问机（Interrogator），来发送询问，应答机接收到询问后，将自动返回飞行器的代号、高度等信息。
