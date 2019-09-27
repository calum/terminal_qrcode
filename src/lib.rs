extern crate qrcode;
extern crate terminal_graphics;

use qrcode::QrCode;
use terminal_graphics::Display;
use terminal_graphics::Colour;

pub fn print_data_as_qr_code() {
    let code = QrCode::new(b"Hello, World!").unwrap();
    let width = code.width();
    let height = (width as f32/2 as f32).ceil() as usize;
    let pixels = code.to_colors();
    let mut display = Display::new((width + 2) as u32, (height + 1) as u32);
    
    for (i, pixel) in pixels.iter().enumerate() {
        let x = i % width;
        let y = (i as f32/width as f32).floor() as isize;

        let colour = match pixel {
            qrcode::types::Color::Light => Colour::White,
            qrcode::types::Color::Dark => Colour::Black,
        };

        let char_colour = match width % 2 {
            0 => colour,
            1 => Colour::White,
            _ => Colour::White,
        };

        match y % 2 {
            0 => display.set_pixel(x as isize + 1, (y/2) as isize + 1, '▄', char_colour, colour),
            1 => display.get_mut_pixel(x as isize + 1, ((y-1)/2) as isize + 1).set_colour(colour),
            _ => println!("That shouldn't happen"),
        }
    }

    for i in 0..((height + 2) as u32) {
        println!();
    }

    display.print();
}
