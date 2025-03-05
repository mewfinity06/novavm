# NovaVM

NovaVM is a virtual machine written in Rust for Novalang

## How to Use

### Preprocessor

```shell
$ cargo run --bin preprocessor asm/input.asm > proj/output.proj
..
```

### Machine

```shell
$ cargo run --bin machine proj/input.proj
| RUNNING THE MACHINE |
..
```

## Sample `.asm` Files

## What's working

### Opcodes

|      Working       |  Name   | Binary Representation | Syntax                      |
| :----------------: | :-----: | :-------------------: | :-------------------------- |
| :heavy_check_mark: |  HALT   |         0x00          | ``HALT``                    |
| :heavy_check_mark: |   NOP   |         0x01          | ``NOP``                     |
|                    | SYSCALL |         0x02          | ``SYSCALL syscall ...args`` |
| :heavy_check_mark: |   ADD   |         0x50          | ``ADD reg v1 v2``           |
| :heavy_check_mark: |   SUB   |         0x51          | ``SUB reg v1 v2``           |
| :heavy_check_mark: |   MUL   |         0x52          | ``MUL reg v1 v2``           |
| :heavy_check_mark: |   DIV   |         0x53          | ``DIV reg v1 v2``           |
| :heavy_check_mark: |  PUSH   |         0x60          | ``PUSH reg``                |
| :heavy_check_mark: |   POP   |         0x61          | ``POP reg``                 |
| :heavy_check_mark: |  SWAP   |         0x62          | ``SWAP reg1 reg2``          |

### Syscalls

| Syscall Number | Name  | Args |
| :------------: | :---: | :--- |
|       1        | EXIT  | NONE |
|       2        | WRITE |      |
