## avr-rust / LLVM float bug

**Update:** I found the source of this bug. LLVM believes the float comparison (generated due to AVR not having floating point hardware) returns a 32-bit value, while it is actually only 8 bits.  
Implementing AVRTargetLowering::getCmpLibcallReturnType() to return MVT::i8 solves this issue.

```rust
    let num = 22.625f32;
    if num < 0.0 {
        // This code executes!
    }
    if 22.625f32 < 0.0 {
        // This code does *not* run, due to optimization.
    }
```

I compile this repo with:  
xargo build --target avr-atmega328p

Everything else in included in this repo.

Tested with avr-rust/rust 6092c82d02b2498b413cbdee5b8617186b70425d
