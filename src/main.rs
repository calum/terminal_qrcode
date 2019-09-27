extern crate qr_terminal;

use qr_terminal::TermQrCode;

fn main() {
    let code = TermQrCode::from_bytes(String::from("https://github.com/calum/terminal_qrcode").as_bytes());
    code.print();
}
