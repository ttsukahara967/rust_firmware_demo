# rust_firmware_demo

Minimal bare-metal Rust firmware for an ARM Cortex-M3, run on a Mac via QEMU (no hardware needed).

## Setup

```bash
brew install qemu
rustup target add thumbv7m-none-eabi
```

## Run

```bash
cargo run --release
```

Expected output:

```
Hello from bare-metal Rust!
SysTick #1
...
SysTick #5
CPUID = 0x410fc231
```

`0x410fc231` is the CPUID of an ARM Cortex-M3 r0p1, read directly from the core's system control registers.

## What to point out

| File | Role |
| --- | --- |
| `src/main.rs` | `#![no_std]` / `#![no_main]` firmware: semihosting output, SysTick timer polling, CPUID register read |
| `memory.x` | Flash / RAM layout of the LM3S6965 that QEMU emulates |
| `.cargo/config.toml` | Cross-compile target, linker script (`link.x` from `cortex-m-rt`), QEMU as the `cargo run` runner |

- `Peripherals::take()` succeeds only once, so the compiler enforces a single owner of each peripheral (moving `cp.SYST` out and then reusing `cp` fails to compile).
- Firmware image size and vector table:

```bash
$(rustc --print sysroot)/lib/rustlib/aarch64-apple-darwin/bin/rust-objcopy -O binary target/thumbv7m-none-eabi/release/fw-demo fw.bin
xxd -l 8 fw.bin   # word 0 = initial SP (0x20010000, top of RAM), word 1 = reset handler (Thumb bit set)
```

The raw image is about 4.7 KB.
