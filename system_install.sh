#!/bin/bash

if [ "$EUID" -ne 0 ]
  then echo "Please run as root"
  exit
fi

systemctl stop hve-cloudinit &&
systemctl stop hve-dhcp &&
systemctl stop hve-server

cp target/release/helio-server /etc/helio/bin/ &&
cp target/release/helio-dhcp /etc/helio/bin/ &&
cp target/release/helio-cloudinit /etc/helio/bin/ &&

cp hve-server.service /etc/systemd/system/ &&
cp hve-dhcp.service /etc/systemd/system/ &&
cp hve-cloudinit.service /etc/systemd/system/ &&

systemctl daemon-reload &&

systemctl enable --now hve-server &&
systemctl enable --now hve-dhcp &&
systemctl enable --now hve-cloudinit