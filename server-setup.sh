# OS configuration for performing benchmarks
#/bin/sh
set -e

cat <<EOF >> /etc/sysctl.conf
fs.file-max=22000500
fs.nr_open=30000500
net.ipv4.tcp_mem='10000000 10000000 10000000'
net.ipv4.tcp_rmem='1024 4096 16384'
net.ipv4.tcp_wmem='1024 4096 16384'
net.core.rmem_max=16384
net.core.wmem_max=16384
net.ipv4.tcp_moderate_rcvbuf=0
net.core.somaxconn=4096
net.core.netdev_max_backlog=4096
net.ipv4.tcp_max_syn_backlog=4096
EOF

cat <<EOF >> /etc/security/limits.conf
root      hard    nofile      30000000
root      soft    nofile      30000000
EOF

# Install build essentials in ubuntu to configure diesel.
yes | sudo apt-get update
yes | sudo apt-get install build-essential sqlite3 libsqlite3-dev curl
yes | sudo apt-get update

# Install Rust
curl https://sh.rustup.rs -sSf | sh

# reboot to configure cargo
reboot

