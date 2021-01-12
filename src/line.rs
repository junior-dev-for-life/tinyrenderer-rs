use crate::render::{ Pixel };

pub fn dda_algorithm(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<Pixel> {
    let x_dist = (x2 - x1);
    let y_dist = (y2 - y1);

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

pub fn line_bresenham(x1: i32, y1: i32, x2: i32, y2: i32) -> Vec<Pixel> {
    let dx = x2 - x1;
    let dy = y2 - y1;
    let mut p = (2 * dy) - dx;
    let mut two_dy = 2 * dy;
    let mut two_dy_dx = 2 * (dy - dx);
    let mut x;
    let mut y;
    let mut x_end;

    match x1 > x2 {
        true => {
            x = x2;
            y = y2;
            x_end = x1;
        },
        false => {
            x = x1;
            y = y1;
            x_end = x2;
        }
    };

    let mut points: Vec<Pixel> = vec![];
    points.push(Pixel { x: x as u32,  y: y as u32 });

    while x < x_end {
        x = x + 1;

        match p < 0 {
            true => { p = p + two_dy; }
            false => {
                y = y + 1;
                p = p + two_dy_dx;
            }
        }

        points.push(Pixel { x: x as u32,  y: y as u32 });
    }

    return points;
}