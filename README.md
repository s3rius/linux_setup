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
pacman -S python python-pip git vim

# Clone this repo
git clone https://github.com/s3rius/linux_setup.git /home/linux_setup
cd /home/linux_setup
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
chmod -R 777 /home/linux_setup
```

Now you need to update vars file according to your needs.
After you're ready just enter

```bash
ansible-playbook -i inventory.ini chroot_playbook.yml

# Reboot your system, log onto your new account
# You can persist your config if you clone it to a different directory.
cd /home/linux_setup
source .venv/bin/activate
# This is config for users.
ansible-playbook -i inventory.ini user_playbook.yml --ask-become-pass
```