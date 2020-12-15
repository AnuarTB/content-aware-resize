extern crate image;

use image::{DynamicImage, GenericImageView, Rgba};

fn euclid_dist(a: Rgba<u8>, b: Rgba<u8>) -> u32 {
  let mut ret: u32 = 0;
  for i in 0..3 {
    ret += (a[i] as i32 - b[i] as i32).pow(2) as u32;
  }
  ret
}

fn error(pos: (u32, u32), img: &DynamicImage) -> f32 {
  let (w, h ) = img.dimensions();
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
          img.get_pixel(nx as u32, ny as u32));
      }
    }
  }
  total_error as f32 / cnt as f32
}

fn main() {
  let img = image::open("surfer.jpg").unwrap();

  println!("dimensions {:?}", img.dimensions());

  println!("{:?}", img.get_pixel(0, 0));

  println!("{:?}", img.get_pixel(0, 1));
  println!("{:?}", img.get_pixel(1, 1));
  println!("{:?}", img.get_pixel(1, 0));

  println!("{}", euclid_dist(img.get_pixel(0, 0), img.get_pixel(0, 1)));
  println!("{}", euclid_dist(img.get_pixel(0, 0), img.get_pixel(1, 1)));
  println!("{}", euclid_dist(img.get_pixel(0, 0), img.get_pixel(1, 0)));


  println!("{}", error((0, 0), &img));
}