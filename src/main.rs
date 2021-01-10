mod obj_reader;
mod render;
mod line;

use render::{ render_to_file, Pixel };
use line::{ dda_algorithm };

const WIDTH: u32 = 500;
const HEIGHT: u32 = 500;

fn main() {
    let obj = obj_reader::read_obj("./obj-files/african-head.obj").unwrap();

    let mut points: Vec<Pixel> = vec![];
    let num_of_faces = obj.faces.len();

    let width = (WIDTH - 1) as f32;
    let height = (HEIGHT - 1) as f32;

    for i in 0..num_of_faces {
        let face = &obj.faces[i];
        for j in 0..3 {
            let v0 = &obj.verts[face[j] as usize];
            let k = (j + 1) % 3;
            let v1 = &obj.verts[face[k] as usize];

            let x0 = ((v0[0] + 1.) * width / 2.) as i32;
            let y0 = ((v0[1] + 1.) * height / 2.) as i32;
            let x1 = ((v1[0] + 1.) * width / 2.) as i32;
            let y1 = ((v1[1] + 1.) * height / 2.) as i32;

            let line = dda_algorithm(x0, y0, x1, y1);
            for pixel in line.iter() {
                points.push(pixel.clone());
            }
        }
    }

    render_to_file(points, WIDTH, HEIGHT, "output.png");
}
