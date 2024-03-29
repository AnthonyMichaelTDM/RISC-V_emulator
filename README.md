# RISCV Emulator

A simple RISC-V emulator written in rust.

Supports a subset of the base RISC-V 32 bit instruction set.

Accepts and runs RISC-V binary files in the ELF format.

only supports the `.text` and `.data` sections

doesn't support compressed binaries

assumes that .data and .text are contiguous in memory, with a small 0x1000 byte gap between them.

## syscall support

Supported syscalls are a supset of those available in RARS.

Notable ommissions include `sbrk`, floating point syscalls, file io syscalls, midi syscalls, `GetCWD`, all dialog calls, and random number generation.

## requirements

besides the obvious, you need to have the riscv toolchain installed. You can use paru to install it from the aur if you're on arch linux, like so:

```bash
sudo paru -S riscv64-unknown-elf-gcc
```

## Getting a RISC-V ELF file

first write your risc-v `.asm` file, then compile it using the riscv toolchain:

```bash
riscv64-unknown-elf-gcc -march=rvim -mabi=ilp32 -x assembler -nostdlib -o <OUTPUT_FILE>.bin <INPUT_FILE>.asm
```

you can use the `riscv64-elf-objdump` tool to see the contents of the file:

```bash
riscv64-unknown-elf-objdump <OUTPUT_FILE>.bin -d
```
