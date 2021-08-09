# riscv-simulator

A riscv simulator implements rv32i and rv64i instruction set written in rust.

## Building

### Windows

To run tests on Windows, you should install mingw on your system. [Msys2](https://www.msys2.org/) is suggested to manage your mingw installation.

Run in msys2 shell:
```shell
pacman -Sy mingw-w64-x86_64-gcc mingw-w64-x86_64-riscv64-unknown-elf-gcc
```

and then add <path to msys2 install>/mingw64/bin to Path environment variable.

and then, you can run `cargo test` at root directory to test this program under windows.
