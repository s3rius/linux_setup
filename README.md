# Archlinux automated configuration

This repo helps you to automate archlinux creation proecss.
This bin is intended to use with UEFI.

> [!IMPORTANT]  
> If you want to use it to store your own config files in your repository,
> you need to fork this one and UPDATE repository in Cargo.toml file.
> 
> It is used to determine where to push cahnges.

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
# Install packages
pacman -S rustup git
# Install stable version of rust.
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

By default after everything is set up, you will be able to find this repo at `$HOME/.config/linux_setup`.
It will be used to sync files with github.

## Updating files

In order to keep your dependencies in sync, you can add them in [consts.rs](./src/consts.rs). It's not updated automatically, because you might want to have some dependencies aside from installation script.

In order to sync dotfiles, you might want to list them in `DOTFILES_MAPPING` in [consts.rs](./src/consts.rs). After that you can sync them with `linux-setup sync`. If you want to commit and push, you can use `linux-setup sync -cp`.

If you want to exclude something, or process files somehow before pushing, you can do it right in `sync_files` function of [main.rs](./src/main.rs).

## Applying config

If you want to pull changes, you can run `linux-setup pull`, or if you have already have the latest version installed, you can just run `linux-setup apply`. It will basically do the same thing as pull, but without pulling changes from the repo.
