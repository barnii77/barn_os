# run using qemu
# this involved copying the file to windows and running it using qemu
# this only works for my machine as the path is hardcoded

# check if it is my machine first by checking the hostname
if [ `hostname` != "LAPTOP-H1NF4KQD" ]; then
    echo "This script is only for my machine"
    exit 1
fi

cargo build --release
# copy the file to windows
cp target/thumbv7em-none-eabihf/release/barn_os /mnt/c/Users/david/barn_os_qemu
