# Archlinux automated configuration

This repo helps you to automate archlinux creation proecss.
This playbook is intended to use with UEFI.

At first you need to create all partitions.
I prefer using `cfdisk /dev/{drive}`

The first partition must be at lest 512Mib and have label as an EFI partition.
After you've created a partition layout, format all drives, mount them, generate fstab and chroot into the root mountpoint.

```bash
# Format partitions.
mkfs.fat -F 32 /dev/sda1
mkswap /dev/sda2
mkfs.ext4 /dev/sda3

# Mount all partitions and enable swap.
mount /dev/sda3 /mnt
mount --mkdir /dev/sda1 /mnt/boot/EFI
swapon /dev/sda2

# Create base archlinux layot
pacstrap -K /mnt base linux linux-firmware base-devel

# Generate fstab file.
genfstab -U /mnt > /mnt/etc/fstab

# Chroot into fresh system
arch-chroot /mnt
```

Now we need to install python and other packages. 

```bash
# Install packages
pacman -S rustup git
rustup install stable

# Clone this repo
git clone https://github.com/s3rius/linux_setup.git /tmp/linux_setup
cd /tmp/linux_setup
# Please adjust this command with CLI argument. See --help for possible config values.
cargo run chroot --username s3rius
```

Once you run this command, you can safely reboot to your newly created system. On your first boot,
use your username and password to log in. On your first login, please run this command to finish setup.

```bash
# Please check `--help` for arguments.
linux-setup user
```

By default after everything is setup, you will be able to find this repo at `$HOME/.config/linux_setup`.
When you do updates to your dotfiles, just run `linux-setup sync` to sync updated files with the repository.
