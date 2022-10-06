extern crate image;

use image::GenericImageView;

fn get_ascii(brightness :u8) -> &'static str {
    let index  = brightness / 32;
    let ascii = [" " , "_" , "." , "," , "-" , "~" , "=" , "+" , ":" , ";" , "c" , "b" , "a" , "!" , "?" , "0" , "1" , "2" , "3" , "4" , "5" , "6" , "7" , "8" , "9" , "$" , "W" , "#" , "@" , "N"];
    return ascii[index as usize];
}

fn main() {
    let image = image::open("/home/danial/Desktop/index.jpeg").unwrap();

    let width = image.dimensions().0;
    let height = image.dimensions().1;

    for y in 0..height {
        for x in 0..width {
            if y % 2 == 0 && x % 2 == 0 {
                let pixel = image.get_pixel(x , y);
                let brightness = pixel[0] / 3 + pixel[1] / 3 + pixel[2] / 3;
                print!("{}" , get_ascii(brightness));
            }
        }
        if y % 2 == 0 {
            println!("");
        }
    }
}
