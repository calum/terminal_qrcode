extern crate qrcode;
extern crate terminal_graphics;

use qrcode::QrCode;
use terminal_graphics::Display;
use terminal_graphics::Colour;

fn main() {
    let code = QrCode::new(b"Hello, World!").unwrap();
    let width = code.width();
    let pixels = code.to_colors();

    let mut display = Display::new(width as u32, (width as f32/2 as f32).ceil() as u32);
    
    for (i, pixel) in pixels.iter().enumerate() {
        let x = (i % width);
        let y = ((i as f32/width as f32).floor() as isize);

        let colour = match pixel {
            qrcode::types::Color::Light => Colour::Black,
            qrcode::types::Color::Dark => Colour::White,
        };

        match y % 2 {
            0 => display.set_pixel(x as isize, (y/2) as isize, '▄', colour, colour),
            1 => display.get_mut_pixel(x as isize, ((y-1)/2) as isize).set_colour(colour),
            _ => println!("That shouldn't happen"),
        }
    }

    display.print();
}
