# run using qemu
# this involved copying the file to windows and running it using qemu
# this only works for my machine as the path is hardcoded

# check if it is my machine first by checking the hostname
if [ `hostname` != "LAPTOP-H1NF4KQD" ]; then
    echo "This script is only for my machine"
    exit 1
fi

# cargo build --release
cargo bootimage
# copy the file to windows
cp target/x86_64-barn_os/debug/bootimage-barn-os.bin /mnt/c/Users/david/bootimage-barn-os.bin
echo 'run using "qemu-system-x86_64 -drive format=raw,file=C:\Users\david\barn_os_qemu\bootimage-barn-os.bin"'