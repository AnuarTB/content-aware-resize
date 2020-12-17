extern crate clap;
extern crate image;

use clap::{App, Arg};
use image::{Rgb, RgbImage};

fn euclid_dist(a: &Rgb<u8>, b: &Rgb<u8>) -> u32 {
    let mut ret: u32 = 0;
    for i in 0..3 {
        ret += (a[i] as i32 - b[i] as i32).pow(2) as u32;
    }
    ret
}

// pos - (row, col)
fn error(pos: (u32, u32), img: &RgbImage) -> f32 {
    let (w, h) = img.dimensions();
    let mut cnt = 0;
    let mut total_error = 0;
    for dy in -1i32..2 {
        for dx in -1i32..2 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let ny = pos.0 as i32 + dy;
            let nx = pos.1 as i32 + dx;
            if ny >= 0 && ny < h as i32 && nx >= 0 && nx < w as i32 {
                cnt += 1;
                total_error += euclid_dist(
                    img.get_pixel(pos.1, pos.0),
                    img.get_pixel(nx as u32, ny as u32),
                );
            }
        }
    }
    total_error as f32 / cnt as f32
}

fn slice_one(img: RgbImage) -> RgbImage {
    let (w, h) = img.dimensions();
    let mut d = vec![vec![0.0f32; w as usize]; h as usize];
    let mut p = vec![vec![0_u8; w as usize]; h as usize];
    for i in 0..w {
        d[0][i as usize] = error((0, i), &img);
    }
    // 0 - Top,
    // 1 - TopLeft
    // 2 - TopRight
    for i in 1_usize..h as usize {
        for j in 0_usize..w as usize {
            d[i][j] = d[i - 1][j];
            if j > 0_usize && d[i - 1][j - 1] < d[i][j] {
                d[i][j] = d[i - 1][j - 1];
                p[i][j] = 1;
            }
            if j + 1 < w as usize && d[i - 1][j + 1] < d[i][j] {
                d[i][j] = d[i - 1][j + 1];
                p[i][j] = 2;
            }
            d[i][j] += error((i as u32, j as u32), &img);
        }
    }
    let mut min_idx = 0;
    for i in 1_usize..w as usize {
        if d[(h - 1) as usize][i] < d[(h - 1) as usize][min_idx] {
            min_idx = i;
        }
    }
    let mut new_img = RgbImage::new(w - 1, h);
    for i in (0_usize..h as usize).rev() {
        let mut off = 0;
        for j in 0_usize..w as usize {
            if j == min_idx {
                off = 1;
                continue;
            }
            new_img.put_pixel(
                (j - off) as u32,
                i as u32,
                img.get_pixel(j as u32, i as u32).clone(),
            );
        }
        if p[i][min_idx] == 1 {
            min_idx -= 1;
        }
        if p[i][min_idx] == 2 {
            min_idx += 1;
        }
    }
    new_img
}

fn main() {
    let matches = App::new("Content aware CLI tool")
        .version("0.1")
        .author("Anuar T. <me@anuartb.com>")
        .about("Resizes your images in a smart way!")
        .arg(
            Arg::with_name("input")
                .short("i")
                .value_name("INPUT_FILE")
                .help("input image path")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .value_name("OUTPUT_FILE")
                .help("output file name")
                .default_value("output.jpg"),
        )
        .arg(
            Arg::with_name("dx")
                .help("By what number of pixels reduce the width of image")
                .required(true),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let mut img: RgbImage = image::open(input_file).unwrap().into_rgb8();

    println!("Dimensions before resizing: {:?}", img.dimensions());

    let iterations = matches.value_of("dx").unwrap().parse::<u32>().unwrap();
    for _ in 0..iterations {
        img = slice_one(img);
    }

    img.save(matches.value_of("output").unwrap()).unwrap();
}
