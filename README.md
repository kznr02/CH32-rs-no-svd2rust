# CH32-RS (WIP)

## Inroduction

This project is attend to implement rust on wch-ch32 microcontroller, now in a very early stage. We don't use svd2rust, it will generate a lot of confusing code just like a piece of sh*t.

Now work well:
- Register Layer
  - [x] PWR
  - [x] RCC
  - [x] GPIO
  - [x] AFIO

## Usage

```shell
  git clone https://github.com/kznr02/CH32-rs
  cd CH32-rs
  cargo build --package ch32-app
  wlink flash target/riscv32imac-unknown-none-elf/debug/ch32-app
```

> Please connect LED on PA0, and build `ch32-app` package, flash with `wlink`, you will see the led on.
> ![image](./doc/led.jpg)
  
