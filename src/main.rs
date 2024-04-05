use serde_json;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use std::error::Error;
use std::collections::HashSet;

fn main() {
    let ew_path = "./ew.json";
    let ew = SquareMatrix::from_file("./ew.json");

    let ee_path = "./ee.json";
    let ee = SquareMatrix::from_file("./ee.json");

    

    
}

#[derive(Debug, PartialEq, Clone)]
struct SquareMatrix {
    size: usize,
    data: Vec<Vec<u8>>
}

impl SquareMatrix {

    /// Make a new matrix of nxn dimensions with all zeros
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

    pub fn to_file(&self, path: &str) {
        let json_string = serde_json::to_string(&self.data).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(json_string.as_bytes()).unwrap();

        println!("Wrote {path}");
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

    pub fn compose(&self, m: &SquareMatrix) -> SquareMatrix {
        // remember that matrix composition is m(self) not self(m)
        // So read m.compose(n) as m composed of n, meaning that we start at
        // n and then move to the graph represented by m

        let s_l: HashSet<(usize, usize)> = self.as_set(); // left hand side
        let s_r: HashSet<(usize, usize)> = m.as_set(); // right hand side

        let mut composed: HashSet<(usize, usize)> = HashSet::new();

        for m in s_r {
            let (x, y) = m;

            for n in &s_l {
                let (a, b) = n;
                if y == *a {
                    composed.insert((x, *b));
                }
            }
        }

        SquareMatrix::from_set(composed, self.size)
    }

    fn trans_clos_rec(m: SquareMatrix, prev: SquareMatrix, original: &SquareMatrix) -> SquareMatrix {
        if m == prev { m }
        else {
            let next = m.compose(&original);
            m.union(&Self::trans_clos_rec(next, m.clone(), original))
        }
    }

    pub fn trans_clos(&self) -> SquareMatrix {
        self.union(&Self::trans_clos_rec(self.clone(), self.clone(), self))
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

    #[test]
    fn test_compose() {
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
            vec![0,0,1],
            vec![0,0,0],
            vec![1,0,1],
        ]);

        let result = m.compose(&n);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_trans_clos() {
        let m = SquareMatrix::from_vec(vec![
            vec![0,0,1],
            vec![0,1,0],
            vec![1,0,1],
        ]);

        dbg!(m.trans_clos());
    }


}

// fn matrix_square(m: SquareMatrix, n: SquareMatrix) -> SquareMatrix {
//     todo!()
// }

// fn matrix_trans(m: SquareMatrix, n: SquareMatrix) -> SquareMatrix {
//     todo!()
// }
