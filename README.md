# RISCV Emulator

A simple RISC-V emulator written in rust.

Supports a subset of the base RISC-V 32 bit instruction set.

Accepts and runs RISC-V binary files in the ELF format.

only supports the `.text` and `.data` sections

doesn't support compressed binaries

I don't know how to initialize the global pointer register, so you need to use the `-mno-relax` flag when compiling your code with the riscv toolchain.

## syscall support

Supported syscalls are a supset of those available in RARS.

Notable ommissions include `sbrk`, floating point syscalls, file io syscalls, midi syscalls, `GetCWD`, all dialog calls, and random number generation.

## requirements

besides the obvious, you need to have the riscv toolchain installed. You can use paru to install it from the aur if you're on arch linux, like so:

```bash
sudo paru -S riscv64-unknown-elf-gcc
```

The entry point for the emulator is `0x0` so make sure your code starts at that address.

## Getting a RISC-V ELF file

first write your risc-v `.asm` file, then compile it using the riscv toolchain:

```bash
riscv64-unknown-elf-gcc -Wl,-Ttext=0x0 -march=rvim -mabi=ilp32 -mno-relax -x assembler -nostdlib -o <OUTPUT_FILE>.bin <INPUT_FILE>.asm
```

you can use the `riscv64-elf-objdump` tool to see the contents of the file:

```bash
riscv64-unknown-elf-objdump <OUTPUT_FILE>.bin -d
```
