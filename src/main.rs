// use serde::{Deserialize, Serialize};
// use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;

fn main() {
    let path = "./ew.json";

    let m = SquareMatrix::from_file(path);
    dbg!(m);
    
}

#[derive(Debug)]
struct SquareMatrix {
    size: usize,
    data: Vec<Vec<u8>>
}

impl SquareMatrix {
    pub fn from_file(path: &str) -> Result<SquareMatrix, Box<dyn Error>> {

        let file = File::open(path)?;
        let reader = BufReader::new(file);
                
        let raw: Vec<Vec<u8>> = serde_json::from_reader(reader)?;
        let matrix: SquareMatrix = SquareMatrix::from_vec(raw);

        Ok(matrix)
    }

    fn from_vec(data: Vec<Vec<u8>>) -> SquareMatrix {
        SquareMatrix {
            size: data.len() as usize,
            data: data
        }
    }
}

// fn matrix_union(m: SquareMatrix, n: SquareMatrix) -> SquareMatrix {
//     todo!()
// }

// fn matrix_intersect(m: SquareMatrix, n: SquareMatrix) -> SquareMatrix {
//     todo!()
// }

// fn matrix_compose(m: SquareMatrix, n: SquareMatrix) -> SquareMatrix {
//     todo!()
// }

// fn matrix_square(m: SquareMatrix, n: SquareMatrix) -> SquareMatrix {
//     todo!()
// }

// fn matrix_trans(m: SquareMatrix, n: SquareMatrix) -> SquareMatrix {
//     todo!()
// }
