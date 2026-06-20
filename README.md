# Archlinux automated configuration

This repo helps you to automate archlinux creation proecss.
It is intended to be used with UEFI.

At first you need to create all partitions.
I prefer using `cfdisk /dev/{drive}`

The first partition must be at lest 512Mib and have label as an EFI partition.
After you've created a partition layout, format all drives, mount them, generate fstab and chroot into the root mountpoint. For reference:

```bash
# Format partitions.
mkfs.fat -F 32 /dev/sda1
mkfs.ext4 /dev/sda2
mkswap /dev/sda3

# Mount all partitions and enable swap.
mount /dev/sda2 /mnt
mount --mkdir /dev/sda1 /mnt/boot/EFI
swapon /dev/sda3

# Create base archlinux layot
pacstrap -K /mnt base linux linux-firmware base-devel

# Generate fstab file.
genfstab -U /mnt > /mnt/etc/fstab

# Chroot into fresh system
arch-chroot /mnt
```

Now we need to install python and other packages. 

```bash
# Install packages for building this project.
pacman -S rustup git
# Install stable version of rust.
rustup install stable

# Clone this repo
git clone https://github.com/s3rius/linux_setup.git /tmp/linux_setup
cd /tmp/linux_setup

# Update config.
vim Config.toml
# Run the installation.
cargo run
```

Once you run this command, you can safely reboot to your newly created system. On your first boot,
use your username and password to log in.
