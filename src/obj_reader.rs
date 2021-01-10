use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

pub struct Obj {
    pub verts: Vec<f32>
}

pub fn read_obj(filename: &str) -> Result<Obj, io::Error> {
    let f = File::open(filename)?;
    let file = BufReader::new(&f);

    let mut verts: Vec<f32> = vec![];

    for line in file.lines() {
        let str = &line.unwrap();

        if str.starts_with("v ") {
            let segments: Vec<&str> = str.split(" ").collect();

            match &segments[..] {
                [_, x, y, z, _] => {
                    verts.push(x.parse::<f32>().unwrap());
                    verts.push(y.parse::<f32>().unwrap());
                    verts.push(z.parse::<f32>().unwrap());
                },
                [_, x, y, z] => {
                    verts.push(x.parse::<f32>().unwrap());
                    verts.push(y.parse::<f32>().unwrap());
                    verts.push(z.parse::<f32>().unwrap());
                },
                _ => panic!("Wrong number of parameters")
            }
        };
    }

    Ok( Obj { verts } )
}