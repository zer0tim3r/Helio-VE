sudo firewall-cmd --permanent --zone=public --add-port=67/udp
sudo firewall-cmd --permanent --zone=public --add-masquerade
sudo firewall-cmd --permanent --zone=public --add-forward

sudo firewall-cmd --permanent --direct --add-rule ipv4 filter FORWARD 0 -s 192.168.10.3 -m mac --mac-source 52:54:00:11:22:33 -j ACCEPT

sudo firewall-cmd --permanent --direct --add-rule ipv4 

sudo firewall-cmd --reload