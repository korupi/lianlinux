<h1 align="center">lianlinux</h1>

<p align="center">
  <a href="https://github.com/korupi/lianlinux/stargazers"><img src="https://img.shields.io/github/stars/korupi/lianlinux?colorA=151515&colorB=B66467&style=for-the-badge&logo=starship"></a>
  <a href="https://github.com/korupi/lianlinux/issues"><img src="https://img.shields.io/github/issues/korupi/lianlinux?colorA=151515&colorB=8C977D&style=for-the-badge&logo=bugatti"></a>
  <a href="https://github.com/korupi/lianlinux/network/members"><img src="https://img.shields.io/github/forks/korupi/lianlinux?colorA=151515&colorB=D9BC8C&style=for-the-badge&logo=github"></a>
</p>

> An app to control Lian Li Hub lights on Linux

> [!WARNING]
> This project is a Work in Process, so expect bugs, etc. I'm working on fixing any bugs and extending functionality of lianlinux.

> [!IMPORTANT]  
> I only have **LianLi-UNI FAN-SL-v1.8** hub, so I can't support other devices. Contributions are welcome!

# About
LianLinux is an app written in Rust to control Lian Li hub RGB light on Linux.

# Installation
Please note that at the moment there's no config system, so unless it's implemented you have to set the colors manually.
```sh
cargo install lian-linux
```

Then, create a file in /etc/udev/rules.d/ called 51-lianlinux.rules with the following content to allow the daemon to access the device:
```
SUBSYSTEM=="hidraw", ATTRS{idVendor}=="0cf2", ATTRS{idProduct}=="a100", MODE="0666", GROUP="plugdev"
```

# Usage
Run the daemon:
```sh
lian-linux daemon
```

Manipulate your controller's lights:
```sh
lian-linux light <MODE> [HEX COLOR]
```

## Example:
Static mode with red color (FF0000):
```sh
lian-linux light static FF0000
```

Runway mode with red and blue colors (FF0000, 0000FF):
```sh
lian-linux light runway FF0000,0000FF
```

## Possible modes
* `static` - requires one HEX color
* `breathing` - requires one HEX color
* `rainbow`
* `morph`
* `runway` - requires two HEX colors

# Roadmap
- [x] Basic working version
- [ ] Configs
- [ ] Support all modes

---
<p align="center>MIT License</p>

