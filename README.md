[install ubuntu apt source ](https://developer.aliyun.com/mirror/ubuntu)

```shell
sudo cp /etc/apt/sources.list /etc/apt/sources.list.old
```

Ubuntu ARM源（ubuntu-ports）: https://developer.aliyun.com/mirror/ubuntu-ports

Ubuntu系统属于哪种版本：

```shell
$ lsb_release -c 
Codename:	focal
```

```shell
 默认注释了源码仓库，如有需要可自行取消注释
deb https://mirrors.aliyun.com/ubuntu-ports/ focal main restricted universe multiverse
# deb-src https://mirrors.aliyun.com/ubuntu-ports/ xenial main main restricted universe multiverse
deb https://mirrors.aliyun.com/ubuntu-ports/ focal-updates main restricted universe multiverse
# deb-src https://mirrors.aliyun.com/ubuntu-ports/ xenial-updates main restricted universe multiverse
deb https://mirrors.aliyun.com/ubuntu-ports/ focal-backports main restricted universe multiverse
# deb-src https://mirrors.aliyun.com/ubuntu-ports/ xenial-backports main restricted universe multiverse
deb https://mirrors.aliyun.com/ubuntu-ports/ focal-security main restricted universe multiverse
# deb-src https://mirrors.aliyun.com/ubuntu-ports/ xenial-security main restricted universe multiverse

# 预发布软件源，不建议启用
# deb https://mirrors.aliyun.com/ubuntu-ports/ xenial-proposed main restricted universe multiverse
# deb-src https://mirrors.aliyun.com/ubuntu-ports/ xenial-proposed main restricted universe multiverse
```

依赖检测
```shell
sudo apt-get install openssh-server
sudo apt install build-essential
sudo apt install cmake git libgtk2.0-dev pkg-config libavcodec-dev libavformat-dev libswscale-dev
sudo apt install python-dev python-numpy libtbb2 libtbb-dev libjpeg-dev libpng-dev libtiff-dev libdc1394-22-dev
```

```shell
sudo apt install -y libopencv-dev clang libclang-dev
```


Install Rust

http://rsproxy.cn/

Install opencv
https://crates.io/crates/opencv

