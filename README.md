# Archlinux automated configuration

This repo helps you to automate archlinux creation proecss.
This playbook is intended to use with UEFI.

At first you need to create all partitions.
I prefer using `cfdisk /dev/{drive}`

The first partition must be at lest 512Mib and have label as an EFI partition.
After you've created a layout, format all drives, mount them, generate fstab and chroot into root mountpoint.

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

Now we need to install python and other needed packages. 
I prefer using pyenv

```bash
# Install packages
pacman -S pyenv git vim

# Enable pyenv
eval "$(pyenv init -)"

# Install python3.9.16
pyenv install 3.9.16
pyenv shell 3.9.16

# Clone this repo
git clone https://github.com/s3rius/linux_setup.git /tmp/linux_setup
mv /tmp/linux_setup
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

Now you need to update vars file according to your needs.
After you're ready just enter

```
ansible-playbook -i inventory.ini playbook.yml
```

This will install everything you need and setup GRUB as default boot manager.