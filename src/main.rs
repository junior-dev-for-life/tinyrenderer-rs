
struct Pixel {
    x: u32,
    y: u32
}

fn main() {
 render(Pixel{ x: 0, y: 0 } );
}



fn render(pixel: Pixel) {
    let width = 800;
    let height = 800;
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
    }

    imgbuf.put_pixel(pixel.x, pixel.y, image::Rgb([255 as u8, 0 as u8, 0 as u8]));
    imgbuf.save("output.png").unwrap();
}
