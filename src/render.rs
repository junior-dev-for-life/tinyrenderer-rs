#[derive(Clone)]
pub struct Pixel {
    pub x: u32,
    pub y: u32
}

pub fn render_to_file(points: Vec<Pixel>, width: u32, height: u32, filepath: &str) {
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
    }

    for point in points.iter() {
        imgbuf.put_pixel(point.x, point.y, image::Rgb([255 as u8, 255 as u8, 255 as u8]));
    }

    image::imageops::flip_vertical_in_place(&mut imgbuf);
    imgbuf.save(filepath).unwrap();
}