# OS configuration for performing benchmarks
# HINT : Use `sudo su -` to run several commands as root user use
# sudo su -
sysctl -w fs.file-max=12000500
sysctl -w fs.nr_open=20000500
ulimit -n 4000000
sysctl -w net.ipv4.tcp_mem='10000000 10000000 10000000'
sysctl -w net.ipv4.tcp_rmem='1024 4096 16384'
sysctl -w net.ipv4.tcp_wmem='1024 4096 16384'
sysctl -w net.core.rmem_max=16384
sysctl -w net.core.wmem_max=16384
echo "root soft nofile 4000000" >> /etc/security/limits.conf
echo "root hard nofile 4000000" >> /etc/security/limits.conf
sysctl -w net.ipv4.ip_local_port_range="1024 64000"

# Install Rust
curl https://sh.rustup.rs -sSf | sh

# Install build essentials in ubuntu to configure diesel.
yes | sudo apt-get update
yes | sudo apt-get install build-essential sqlite3 libsqlite3-dev 

# Setup the project
cargo install diesel_cli --no-default-features --features sqlite
echo "DATABASE_URL=test.db" > .env
diesel migration run

# ##########################################################################################################
# 
# Steps to set permanently ulimit -n / open files in ubuntu
# Credits: https://medium.com/@muhammadtriwibowo/set-permanently-ulimit-n-open-files-in-ubuntu-4d61064429a
#
# ##########################################################################################################

# # To increase the available limit to say 4000000
# sudo vim /etc/sysctl.conf

# # add the following line to it
# fs.file-max = 4000000

# # run this to refresh with new config
# sudo sysctl -p

# # edit the following file
# sudo vim /etc/security/limits.conf

# # add following lines to it
# * soft     nproc          4000000    
# * hard     nproc          4000000   
# * soft     nofile         4000000   
# * hard     nofile         4000000
# root soft     nproc          4000000    
# root hard     nproc          4000000   
# root soft     nofile         4000000   
# root hard     nofile         4000000

# # edit the following file
# sudo vim /etc/pam.d/common-session

# # add this line to it
# session required pam_limits.so

# # logout and login and try the following command
# ulimit -n
# 4000000