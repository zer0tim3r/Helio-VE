#!/bin/bash

if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


mkdir -p /etc/helio/bin
mkdir -p /etc/helio/disks
mkdir -p /etc/helio/images
mkdir -p /etc/helio/pids
mkdir -p /etc/helio/socket
brctl addbr br0
ip tuntap add dev tap0 mode tap
brctl addif br0 tap0

echo "
auto br0
iface br0 inet static
  bridge_ports tap0
  address 192.168.10.254/24

auto tap0
iface tap0 inet manual
" > /etc/network/interfaces.d/helio

sysctl -w net.ipv4.ip_forward=1

systemctl restart networking

mkdir -p /etc/qemu

echo "allow br0
deny *" > /etc/qemu/bridge.conf