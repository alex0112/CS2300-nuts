use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::collections::HashSet;

fn main() {
    // let path = "./ew.json";

    // let m = SquareMatrix::from_file(path);
    // dbg!(m);
    
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

    pub fn from_vec(data: Vec<Vec<u8>>) -> SquareMatrix {
        SquareMatrix {
            size: data.len() as usize,
            data: data
        }
    }

    pub fn as_set(&self) -> HashSet<(usize, usize)> {
        let mut res: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..(self.size)as usize {
            for j in 0..(self.size) as usize {
                if self.data[i][j] == 1 {
                    dbg!("equal!");
                    res.insert((i,j));

                }
            }
        }

        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_set() {
        let m = SquareMatrix::from_vec(vec![
            vec![0,0,1],
            vec![0,1,0],
            vec![1,0,1],
        ]);

        let expected: HashSet<(usize, usize)> = HashSet::from([(0,2), (1,1), (2,0), (2,2)]);
        assert_eq!(m.as_set(), expected);
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
