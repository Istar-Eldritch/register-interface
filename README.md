

This crate adds two proc macros:
- `field`: Allows to represent a bitfield of a register.
- `register`: Allows to represent a register on a register map.

Usage
---

To represent a new register type, define a new struct and anotate its differnet bitfield sections:

```rs
// (name_of_field, position from, position to)
#[field(data, 0, 7)]
#[field(full, 31, 31)]
pub struct TxData {
    addr: *mut usize,
}
```

To represent a device register map, define a new struct and anotate its didfferent registers.

```rs
// (name of register, type of register, offset from base address)
#[register(txdata, TxData, 0x0)]
#[register(rxdata, RxData, 0x4)]
#[register(txctrl, TxCtrl, 0x08)]
#[register(rxctrl, RxCtrl, 0x0C)]
#[register(ie, InterruptRegister, 0x10)]
#[register(ip, InterruptRegister, 0x14)]
#[register(div, Div, 0x18)]
pub struct Uart {
    addr: *mut usize,
}
```

To access the fields you can then:

```rs
let mut uart = Uart::new(&mut reg_ptr);
let data = uart.rxdata.data();
uart.rxdata().set_data(data + 1);
```

Overhead
---
Both reads and writes perform two bitwise operations, a mask and a bitshift on top of the read / write operations.

