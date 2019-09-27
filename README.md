# Terminal QR Code

[![crates.io](https://img.shields.io/crates/v/qr_terminal.svg)](https://crates.io/crates/qr_terminal)

Generates a QR code in your terminal for CLI tools.

```rust
extern crate qr_terminal;

use qr_terminal::TermQrCode;

fn main() {
    let code = TermQrCode::from_bytes(b"Hello, World!");
    code.print();
}
```
