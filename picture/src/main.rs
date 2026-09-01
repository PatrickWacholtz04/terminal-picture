use image::{GenericImageView, ImageReader, Pixel};
use std::{error::Error};
// use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "../picture/images/image.jpg"; // Hard code test image
    // Define terminal output size
    const TERM_W: usize = 16;
    const TERM_H: usize = 16;

    let mut screen_buf: [[(usize, usize, usize); TERM_H]; TERM_W] = [[(0, 0, 0); TERM_H]; TERM_W];

    // Load image from file path and get its dimensions
    let image = ImageReader::open(path)?
        .decode()?;
    let (img_w, img_h) = image.dimensions();

    // Define a block width/height so that we can scale image down to terminal dimensions
    let (block_w, block_h) = (img_w as usize / TERM_W, img_h as usize / TERM_H);

    // DEBUG Prints
    print!("Terminal: {} x {}/2 chars\n", TERM_W, TERM_H);
    print!("   Image: {} x {} pxl \n", img_w, img_h);
    print!("   Block: {} x {} pxl\n", block_w, block_h);


    for block_col in 0..TERM_W { 
        for block_row in 0..TERM_H {
            let start_x = block_col * img_w as usize / TERM_W;
            let end_x = (block_col+1) * img_w as usize / TERM_W;

            let start_y = block_row * img_h as usize / TERM_H;
            let end_y = (block_row+1) * img_h as usize / TERM_H;

            let mut avg_r = 0usize;
            let mut avg_g = 0usize;
            let mut avg_b = 0usize;
            let mut count = 0usize;

            for x in start_x..end_x {
                for y in start_y..end_y {
                    let pixel = image.get_pixel(x as u32, y as u32);
                    let rgb = pixel.to_rgb();

                    avg_r += rgb.0[0] as usize;
                    avg_g += rgb.0[1] as usize;
                    avg_b += rgb.0[2] as usize;
                    count += 1;
                }
            }

            let avg_rgb = (avg_r / count, avg_g / count, avg_b / count);
            screen_buf[block_col][block_row] = avg_rgb;

        }
    }

    // Display screen_buf to console.
    let pix = '▄';

    for row in (0..TERM_H).step_by(2) {
        // Each char will hold an upper and a lower pixel, so 2 rows
        for col in 0..TERM_W {
            let (r_top, g_top, b_top) = screen_buf[col][row];

            let mut rgb_btm = (0, 0, 0);
            if row + 1 < TERM_H {
                rgb_btm = screen_buf[col][row+1];   
            }

            let (r_btm, g_btm, b_btm) = rgb_btm;   
            
            // Background Color (Top Pixel)
            // Foreground Color (Bottom Pixel)
            // Pixel Character (▄)
            // Reset Color(s)
            print!("\x1b[48;2;{};{};{}m\
                    \x1b[38;2;{};{};{}m\
                    {}\
                    \x1b[0m", 
                    r_top, g_top, b_top, 
                    r_btm, g_btm, b_btm, 
                    pix
                );
        }
        println!();
    }

    Ok(())
}

