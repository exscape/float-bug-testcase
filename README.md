## avr-rust / LLVM float bug

```rust
    let num = 22.625f32;
    if num < 0.0 {
        // This code executes!
    }
    if 22.625f32 < 0.0 {
        // This code does *not* run, perhaps due to optimization? See disassembly below.
    }
```

The second if statement seems to disassemble to what is a fairly complex no-op, if I understand the assembly correctly:

```
0000074a <LBB10_13>:
 74a:   81 e0           ldi     r24, 0x01       ; 1
 74c:   80 30           cpi     r24, 0x00       ; 0
 74e:   39 f4           brne    .+14            ; 0x75e <LBB10_15>
 750:   00 c0           rjmp    .+0             ; 0x752 <LBB10_14>

00000752 <LBB10_14>:
 752:   88 e2           ldi     r24, 0x28       ; 40
 754:   90 e0           ldi     r25, 0x00       ; 0
 756:   61 e0           ldi     r22, 0x01       ; 1
 758:   0e 94 7e 02     call    0x4fc   ; 0x4fc <_ZN4core3ptr14write_volatile17h59dd1221bcfb7931E>
 75c:   01 c0           rjmp    .+2             ; 0x760 <LBB10_16>

0000075e <LBB10_15>:
 75e:   0d c0           rjmp    .+26            ; 0x77a <LBB10_20>

00000760 <LBB10_16>:
 760:   0e 94 df 03     call    0x7be   ; 0x7be <_ZN18float_bug_testcase11small_delay17hda1f3c2f2ff48fc7E>
 764:   00 c0           rjmp    .+0             ; 0x766 <LBB10_17>

00000766 <LBB10_17>:
 766:   88 e2           ldi     r24, 0x28       ; 40
 768:   90 e0           ldi     r25, 0x00       ; 0
 76a:   6f ef           ldi     r22, 0xFF       ; 255
 76c:   0e 94 7e 02     call    0x4fc   ; 0x4fc <_ZN4core3ptr14write_volatile17h59dd1221bcfb7931E>
 770:   00 c0           rjmp    .+0             ; 0x772 <LBB10_18>

00000772 <LBB10_18>:
 772:   0e 94 df 03     call    0x7be   ; 0x7be <_ZN18float_bug_testcase11small_delay17hda1f3c2f2ff48fc7E>
 776:   00 c0           rjmp    .+0             ; 0x778 <LBB10_19>

00000778 <LBB10_19>:
 778:   00 c0           rjmp    .+0             ; 0x77a <LBB10_20>

0000077a <LBB10_20>:
 77a:   0e 94 44 04     call    0x888   ; 0x888 <_ZN18float_bug_testcase11large_delay17hcdf7b2c3f9709c05E>
 77e:   00 c0           rjmp    .+0             ; 0x780 <LBB10_21>
```

I compile this repo with:  
xargo build --target avr-atmega328p

Everything else in included in this repo.

Tested with avr-rust/rust 6092c82d02b2498b413cbdee5b8617186b70425d
