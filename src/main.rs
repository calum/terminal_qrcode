extern crate qr_terminal;

use qr_terminal::TermQrCode;

fn main() {
    let code = TermQrCode::from_bytes(b"Hello, World!");
    code.print();
}
