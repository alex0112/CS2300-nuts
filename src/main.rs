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

#[derive(Debug, PartialEq)]
struct SquareMatrix {
    size: usize,
    data: Vec<Vec<u8>>
}

impl SquareMatrix {

    /// Make a new matrix of nxn dimensions of all zeros
    pub fn new(size: usize) -> SquareMatrix {
        let mut data: Vec<Vec<u8>> = vec![];

        for _i in 0..size {

            let mut row: Vec<u8> = vec![];
            for _j in 0..size {
                row.push(0u8);
            }
            data.push(row);
        }

        SquareMatrix {
            size: size,
            data: data,
        }
    }

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

    pub fn from_set(set: HashSet<(usize, usize)>, matrix_size: usize) -> SquareMatrix {
        let mut data: Vec<Vec<u8>> = vec![];

        for i in 0..matrix_size {
            let mut row: Vec<u8> = vec![];

            for j in 0..matrix_size {
                if set.contains(&(i, j)) {
                    row.push(1u8);
                } else {
                    row.push(0u8);
                }
            }

            data.push(row);
        }

        SquareMatrix { size: matrix_size, data: data }
    }

    pub fn as_set(&self) -> HashSet<(usize, usize)> {
        let mut res: HashSet<(usize, usize)> = HashSet::new();

        for i in 0..(self.size)as usize {
            for j in 0..(self.size) as usize {
                if self.data[i][j] == 1 {
                    res.insert((i, j));
                }
            }
        }

        res
    }

    pub fn union(&self, m: &SquareMatrix) -> SquareMatrix {
        let s1 = self.as_set();
        let s2 = m.as_set();

        let union: HashSet<(usize, usize)> = s1.union(&s2).cloned().collect();

        SquareMatrix::from_set(union, self.size)
    }

    pub fn intersect(&self, m: &SquareMatrix) -> SquareMatrix {
        let s1 = self.as_set();
        let s2 = m.as_set();

        let intersection: HashSet<(usize, usize)> = s1.intersection(&s2).cloned().collect();

        SquareMatrix::from_set(intersection, self.size)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = SquareMatrix::new(3);

        let expected = SquareMatrix {
            size: 3,
            data: vec![
                vec![0,0,0],
                vec![0,0,0],
                vec![0,0,0],
            ],
        };

        assert_eq!(m, expected);
    }

    #[test]
    fn test_from_set() {
        let relation: HashSet<(usize, usize)> = HashSet::from([(0,2), (1,1), (2,0), (2,2)]);

        let expected = SquareMatrix::from_vec(vec![
            vec![0,0,1],
            vec![0,1,0],
            vec![1,0,1],
        ]);

        let res = SquareMatrix::from_set(relation, 3);

        assert_eq!(res, expected);
    }

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

    #[test]
    fn test_union() {
        let m = SquareMatrix::from_vec(vec![
            vec![0,0,1],
            vec![0,1,0],
            vec![1,0,1],
        ]);

        let n = SquareMatrix::from_vec(vec![
            vec![1,0,0],
            vec![0,0,0],
            vec![0,0,1],
        ]);

        let expected = SquareMatrix::from_vec(vec![
            vec![1,0,1],
            vec![0,1,0],
            vec![1,0,1],
        ]);

        let result = m.union(&n);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_intersect() {
        let m = SquareMatrix::from_vec(vec![
            vec![0,0,1],
            vec![0,1,0],
            vec![1,0,1],
        ]);

        let n = SquareMatrix::from_vec(vec![
            vec![1,0,0],
            vec![0,0,0],
            vec![1,0,1],
        ]);

        let expected = SquareMatrix::from_vec(vec![
            vec![0,0,0],
            vec![0,0,0],
            vec![1,0,1],
        ]);

        let result = m.intersect(&n);

        assert_eq!(result, expected);
    }



}

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
