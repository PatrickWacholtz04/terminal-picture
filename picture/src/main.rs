use image::{GenericImageView, ImageReader, Pixel};
use std::{error::Error, fs::File, io::Write, env};

const DEFAULT_W:usize = 16;
const DEFAULT_H:usize = 16;
const DEFAULT_IMAGE_SIZE:usize = 250;


fn main() -> Result<(), Box<dyn Error>> {
        // Define terminal output size

    let args: Vec<String> = env::args().collect();
    let term_w: usize = args
        .get(1)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_W);

    let term_h: usize = args
        .get(2)
        .and_then(|arg| arg.parse().ok())
        .unwrap_or(DEFAULT_H);

    let manual = args
        .iter()
        .any(|arg| arg == "manual");

    let image_size: usize = args
        .iter()
        .find_map(|arg| {
            arg.strip_prefix("image:")
                .and_then(|value| value.parse().ok())
        })
        .unwrap_or(DEFAULT_IMAGE_SIZE);



    if manual == false {
        let resp = reqwest::blocking::get(format!("https://picsum.photos/{image_size}"))?;
        let bytes = resp.bytes()?;

        let mut file = File::create("../picture/images/image.jpg")?;
        file.write_all(&bytes)?;
    }
    
    let path = "../picture/images/image.jpg"; // Hard code test image

    let mut screen_buf = vec![vec![(0usize, 0usize, 0usize); term_h]; term_w];

    // Load image from file path and get its dimensions
    let image = ImageReader::open(path)?
        .decode()?;
    let (img_w, img_h) = image.dimensions();

    // Define a block width/height so that we can scale image down to terminal dimensions
    let (block_w, block_h) = (img_w as usize / term_w, img_h as usize / term_h);

    // DEBUG Prints
    print!("Terminal: {} x ⌈{}/2⌉ chars\n", term_w, term_h);
    print!("   Image: {} x {} pxl \n", img_w, img_h);
    print!("   Block: {} x {} pxl\n", block_w, block_h);


    for block_col in 0..term_w { 
        for block_row in 0..term_h {
            let start_x = block_col * img_w as usize / term_w;
            let end_x = (block_col+1) * img_w as usize / term_w;

            let start_y = block_row * img_h as usize / term_h;
            let end_y = (block_row+1) * img_h as usize / term_h;

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

    for row in (0..term_h).step_by(2) {
        // Each char will hold an upper and a lower pixel, so 2 rows
        for col in 0..term_w {
            let (r_top, g_top, b_top) = screen_buf[col][row];

            let mut rgb_btm = (0, 0, 0);
            if row + 1 < term_h {
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