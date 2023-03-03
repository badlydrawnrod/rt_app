# rt_app

A simple app that uses the [simple RISC-V runtime](https://github.com/badlydrawnrod/rt) for Arviss.

# Building

- Install the [simple RISC-V runtime](https://github.com/badlydrawnrod/rt) and its prerequisites as described in its [README](https://github.com/badlydrawnrod/rt/blob/master/README.md).

- `git clone` this code into a parallel directory

```
$ git clone https://github.com/badlydrawnrod/rt_app
```

## Perform a release build

Change directory into `rt_app` and perform a release build.

- Perform a release build

```
$ cd rt_app
$ cargo build --release
```

## Confirm that the build created a RISCV-V binary

- Disassemble it to confirm that it has created a RISC-V binary

Use `cargo objdump` to disassemble the resulting binary. The example below shows a disassembly of the `.init`
section, which is effectively the minimal runtime that performs initialization before jumping to `main()`.

```
$ cargo objdump --release -- --disassemble --section=.init
    Finished release [optimized] target(s) in 0.00s

rt_app: file format elf32-littleriscv

Disassembly of section .init:

00000000 <_start>:
       0: 97 51 00 00   auipc   gp, 5
       4: 93 81 01 80   addi    gp, gp, -2048

00000008 <.Lpcrel_hi1>:
       8: 17 81 00 00   auipc   sp, 8
       c: 13 01 81 ff   addi    sp, sp, -8
      10: 33 04 01 00   add     s0, sp, zero

00000014 <.Lpcrel_hi2>:
      14: 17 45 00 00   auipc   a0, 4
      18: 13 05 c5 fe   addi    a0, a0, -20

0000001c <.Lpcrel_hi3>:
      1c: 97 45 00 00   auipc   a1, 4
      20: 93 85 45 fe   addi    a1, a1, -28
      24: 13 06 00 00   li      a2, 0

00000028 <clear_bss>:
      28: 63 78 b5 00   bgeu    a0, a1, 0x38 <finish_bss>
      2c: 23 00 c5 00   sb      a2, 0(a0)
      30: 13 05 15 00   addi    a0, a0, 1
      34: e3 0a 00 fe   beqz    zero, 0x28 <clear_bss>

00000038 <finish_bss>:
      38: 97 00 00 00   auipc   ra, 0
      3c: e7 80 c0 00   jalr    12(ra)
      40: 73 00 10 00   ebreak
```

## Extract an image

Extract an image using `cargo objcopy`. This will generate a binary file `app.bin` in the root of the crate. It is
not useful by itself, but will need to be run on [a compatible emulator](https://github.com/badlydrawnrod/arviss_experiment).

- Extract an image

This extracts just the required sections to make the extracted image very small.

```
$ cargo objcopy --release -- -O binary app.bin -j .init -j .text -j .rodata -j .data
```

Eventually the simulator may also support loading ELF much like [C-Arviss](https://github.com/badlydrawnrod/arviss)
does.
