
fn main() {
 render();
}

fn render() {
    let width = 800;
    let height = 800;
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([255 as u8, 255 as u8, 255 as u8]);
    }

    imgbuf.save("output.png").unwrap();
}
