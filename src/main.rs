struct Pixel {
    x: u32,
    y: u32
}

fn main() {
    let dda_points = dda_algorithm(90 , 80, 60, 50);

    render(dda_points);
}

fn dda_algorithm(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<Pixel> {
    let x_dist = x2 - x1;
    let y_dist = y2 - y1;
    let steps = if x_dist.abs() > y_dist.abs() { x_dist.abs() } else { y_dist.abs() };
    let x_increment = x_dist / steps;
    let y_increment = y_dist / steps;

    let mut x = x1;
    let mut y = y1;
    let mut points: Vec<Pixel> = vec![];
    points.push(Pixel { x: x as u32,  y: y as u32 });

    for i in 1..=steps {
        x += x_increment;
        y += y_increment;
        points.push(Pixel{ x: x as u32, y: y as u32 });
    }

    return points;
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
    imgbuf.save("output_dda_reverse.png").unwrap();
}
