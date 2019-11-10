use std::fs::File;
use std::io::BufWriter;
use std::convert::TryInto;

use png;
use amethyst::utils::application_root_dir;

use std::fs;
use serde::Deserialize;

#[derive(Copy, Clone, Deserialize, Default)]
pub struct Config{
    pub sheets: usize,
    pub textures: usize,
    pub width: usize,
    pub height: usize,
}

pub fn generate(snum: usize, tnum: usize, w_d: usize, h_d: usize) -> amethyst::Result<()>{
    let root = application_root_dir()?;
    let textures = root.join("res").join("textures");

    let textures = textures.to_str().unwrap();

    for s in 0..snum {
        let file = File::create(format!("{}{}{}{}", textures, "\\spritesheet_", s, ".png")).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, w_d as u32, (tnum * h_d) as u32); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::RGBA);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        let size: usize = tnum * w_d * h_d * 4;
        println!("{}", size);
        let mut data = vec![0; size]; 
        for i in 0..tnum {
            println!("Making {} of {} on spritesheet {}", i, tnum, s);
            let base_num: usize = (i * (w_d * h_d)).try_into().unwrap();
            println!("{}", base_num);
            for h_p in 0..h_d {
                for w_p in 0..w_d {
                    data[base_num + (h_p * w_d + w_p) + 0] = 0;
                    data[base_num + (h_p * w_d + w_p) + 1] = 255;
                    data[base_num + (h_p * w_d + w_p) + 2] = 0;
                    data[base_num + (h_p * w_d + w_p) + 3] = 255;
                }
            }
        }
        writer.write_image_data(&data).unwrap();
    }
    Ok(())
}