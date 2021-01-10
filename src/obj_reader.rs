use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

pub struct Obj {
    pub verts: Vec<Vec<f32>>,
    pub faces: Vec<Vec<i32>>
}

pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

pub fn read_obj(filename: &str) -> Result<Obj, io::Error> {
    let f = File::open(filename)?;
    let file = BufReader::new(&f);

    let mut verts: Vec<Vec<f32>> = vec![];
    let mut faces: Vec<Vec<i32>> = vec![];

    for line in file.lines() {
        let str = &line.unwrap();

        if str.starts_with("v ") {
            let segments: Vec<&str> = str.split(" ").collect();

            match &segments[..] {
                [_, x, y, z, _] => {
                    verts.push(vec![
                        x.parse::<f32>().unwrap(),
                        y.parse::<f32>().unwrap(),
                        z.parse::<f32>().unwrap()
                    ]);
                },
                [_, x, y, z] => {
                    verts.push(vec![
                        x.parse::<f32>().unwrap(),
                        y.parse::<f32>().unwrap(),
                        z.parse::<f32>().unwrap()
                    ]);
                },
                _ => panic!("Wrong number of parameters")
            }
        } else if str.starts_with("f ") {
            let segments: Vec<&str> = str.split(" ").collect();
            match &segments[..] {
                [_, x, y, z] => {
                    let mut face: Vec<i32> = vec![];
                    let first = x.split("/").collect::<Vec<&str>>();
                    let second = y.split("/").collect::<Vec<&str>>();
                    let third = z.split("/").collect::<Vec<&str>>();

                    face.push(first[0].parse::<i32>().unwrap() - 1);
                    face.push(second[0].parse::<i32>().unwrap() - 1);
                    face.push(third[0].parse::<i32>().unwrap() - 1);

                    faces.push(face);
                },
                _ => panic!("Wrong face line")
            }
        };
    }

    Ok( Obj { verts, faces } )
}