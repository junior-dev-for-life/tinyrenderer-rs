struct Pixel {
    x: u32,
    y: u32
}

fn main() {
    let points = line(13 , 20, 80, 40);
    render(points);
}

fn line(x1: u32, y1: u32, x2: u32, y2: u32) -> Vec<Pixel> {
    let mut output = vec![];

    let mut i = 0.0;

    while i <= 1.0 {
        let x = (((x1 + (x2 - x1)) as f32) * i) as u32;
        let y = (((y1 + (y2 - y1)) as f32) * i) as u32;
        output.push(Pixel { x, y });
        i = i + 0.01;
    }

    return output;
}

fn render(points: Vec<Pixel>) {
    let width = 100;
    let height = 100;
    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([0 as u8, 0 as u8, 0 as u8]);
    }

    for point in points.iter() {
        imgbuf.put_pixel(point.x, point.y, image::Rgb([255 as u8, 255 as u8, 255 as u8]));
    }

    image::imageops::flip_vertical_in_place(&mut imgbuf);
    imgbuf.save("output.png").unwrap();
}
