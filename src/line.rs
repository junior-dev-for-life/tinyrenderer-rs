use crate::render::{ Pixel };

pub fn dda_algorithm(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<Pixel> {
    let x_dist = x2 - x1;
    let y_dist = y2 - y1;
    let steps = if x_dist.abs() > y_dist.abs() { x_dist.abs() } else { y_dist.abs() };
    let x_increment = x_dist as f32 / steps as f32;
    let y_increment = y_dist as f32 / steps as f32;

    let mut x = x1 as f32;
    let mut y = y1 as f32;
    let mut points: Vec<Pixel> = vec![];
    points.push(Pixel { x: x as u32,  y: y as u32 });

    for _i in 1..=steps {
        x += x_increment;
        y += y_increment;
        points.push(Pixel{ x: x as u32, y: y as u32 });
    }

    return points;
}