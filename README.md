An [`embedded-hal`](https://docs.rs/embedded-hal/latest/embedded_hal/) focused driver for the TCA9548 I2C multiplexer.

This crate allows you to use the multiplexed I2C busses on the TCA9548 as if they were separate I2C busses.

## Portable Atomic
This crate uses [`portable-atomic`](https://docs.rs/portable-atomic/latest/portable_atomic/) to provide platform-agnostic atomic operations. This is necessary to implement the internal shared i2c bus. You may need to enable certain features of `portable-atomic` to get this crate to compile on platforms that don't natively support atomic operations.

## Usage

```rust
// 0x70 is used here, the actual address will vary
// depending on the configuration of the ADDR
// pins on the chip.
let mut multiplexer = Tca9548::new(i2c, 0x70).unwrap();

// This borrows the multiplexer, so the multiplexer
// cannot be dropped before it's busses.
let tca9548_eh::Busses {
    bus0,
    bus1,
    ..
} = multiplexer.split();

bus0.write(0x38, &[0x00]).unwrap();
```
