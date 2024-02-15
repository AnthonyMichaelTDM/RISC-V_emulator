# RISCV Emulator

A simple RISC-V emulator written in rust.

Supports a subset of the base RISC-V 32 bit instruction set.

Accepts and runs RISC-V binary files in the ELF format.

only supports the `.text` and `.data` sections

## requirements

besides the obvious, you need to have the riscv toolchain installed. You can use pacman to install it:

```bash
sudo pacman -S riscv64-elf-gcc
```

The entry point for the emulator is `0x0` so make sure your code starts at that address.

## Getting a RISC-V ELF file

first write your risc-v `.asm` file, then compile it using the riscv toolchain:

```bash
riscv64-elf-as <INPUT_FILE>.asm -o <OUTPUT_FILE>.bin -march=rv32im
```
