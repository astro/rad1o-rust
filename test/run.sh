#!/usr/bin/env bash

set -e

cargo build --release

(
    set -e

    cd target/thumbv7em-none-eabihf/release
    arm-none-eabi-objcopy -O binary --strip-unneeded test test.c1d
    ls -l test test.c1d

    sudo mount /dev/disk/by-id/usb-NXP_LPC_Mem_Disk_123456789ABCDEF-0:0 /mnt/sd/
    sudo cp test.c1d /mnt/sd/
    sudo umount /mnt/sd/
    sync
    echo Copied
)
