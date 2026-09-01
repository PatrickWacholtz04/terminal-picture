use image::{GenericImageView, ImageReader, Pixel, Rgb};
use std::error::Error;
// use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "../picture/images/image.jpg"; // Hard code test image
    // Define terminal output size
    const TERM_W: usize = 16;
    const TERM_H: usize = 16;

    let mut screen_buf: [[(usize, usize, usize); TERM_W]; TERM_H] = [[(0, 0, 0); TERM_W]; TERM_H];

    // Load image from file path and get its dimensions
    let image = ImageReader::open(path)?
        .decode()?;
    let (img_w, img_h) = image.dimensions();

    // Define a block width/height so that we can scale image down to terminal dimensions
    let (block_w, block_h) = (img_w as usize / TERM_W, img_h as usize / TERM_H);

    // DEBUG Prints
    print!("Terminal: {} {}\n", TERM_W, TERM_H);
    print!("   Image: {} {}\n", img_w, img_h);
    print!("   Block: {} {}\n", block_w, block_h);


    let mut block_col = 0;
    // Iterate through image in blocks and get average color value of each block
    for col in (0..img_w-block_w as u32).step_by(block_w) {
        let mut block_row = 0;
        for row in (0..img_h-block_h as u32).step_by(block_h) {

            // Keep list of all pixel colors in block
            let mut colors: Vec<Rgb<u8>> = Vec::new();
            for x in 0..block_w {
                for y in 0..block_h {
                    let pixel = image.get_pixel(col + x as u32, row + y as u32);
                    colors.push( pixel.to_rgb() );
                }
            }

            // Get average RGB value of block
            let mut avg_r:usize = 0;
            let mut avg_g:usize = 0;
            let mut avg_b:usize = 0;

            for i in &colors {
                avg_r += i.0[0] as usize;
                avg_g += i.0[1] as usize;
                avg_b += i.0[2] as usize;
            }

            avg_r /= colors.len();
            avg_g /= colors.len();
            avg_b /= colors.len();

            // Convert RGB to ANSI 216 using Color Cube method
            let r_level = (avg_r as f32 / 256.0) * 6.0;
            let g_level = (avg_g as f32 / 256.0) * 6.0;
            let b_level = (avg_b as f32 / 256.0) * 6.0;

            let ansi_code_float = 16.0 + (36.0 * r_level) + (6.0 * g_level) + b_level;
            let ansi_code = ansi_code_float as u8;

            screen_buf[block_col][block_row] = (avg_r, avg_g, avg_b);

            block_row += 1;
        }
        block_col += 1;
    }


    // Display screen_buf to console.
    let pix = '▄';

    for row in (0..screen_buf[0].len()).step_by(2) {
        // Each char will hold an upper and a lower pixel, so 2 rows
        for col in 0..screen_buf[0].len() {
            // let foreground = "\x1b[38;5;" + screen_buf[col][row];
            let (r1, g1, b1) = screen_buf[col][row];
            let (r2, g2, b2) = screen_buf[col][row+1];


            // print!("\x1b[48;5;{}m\x1b[38;5;{}m{}\x1b[0m", screen_buf[col][row], , pix);

            print!("\x1b[48;2;{};{};{}m\x1b[38;2;{};{};{}m{}\x1b[0m", r1, g1, b1, r2, g2, b2, pix);

        }
        println!();
    }


    Ok(())
}
