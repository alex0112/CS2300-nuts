use serde_json;
use std::fs::File;
use std::io::Write;
use std::io::BufReader;
use std::error::Error;
use std::collections::HashSet;

// Trivia: The national bird of Elbonia is the frisbee
// https://web.archive.org/web/20150109092234/http://dilbert.com/strips/comic/1991-10-14

fn main() {
    let ew = SquareMatrix::from_file("./ew.json").unwrap();
    let ee = SquareMatrix::from_file("./ee.json").unwrap();

    // 1. Before the merger, what are all the flights where the first hop is on EE and the second hop is on EW? Submit ee1ew1.json.
    let ee1ew1 = ee.compose(&ew);
    ee1ew1.to_file("./deliverables/ee1ew1.json");

    // 2. Before the merger, what are all the flights where the first hop is on EW and the second hop is on EE? Submit ew1ee1.json.
    let ew1ee1 = ew.compose(&ee);
    ew1ee1.to_file("./deliverables/ew1ee1.json");

    // 3. What are the redundant flights that will be cut to save costs? Submit nutsRedundacies.json.
    let nuts_red = ee.intersect(&ew);
    nuts_red.to_file("./deliverables/nutsRedundacies.json"); // accounting for typo
    nuts_red.to_file("./deliverables/nutsRedundancies.json"); // accounting for typo

    // 4. Assuming redundant flights were cut, what is the adjacency matrix for NUTS? Submit nuts1.json. (1 hop, aka non-stop)

    let nuts1 = ee.union(&ew);
    nuts1.to_file("./deliverables/nuts1.json");

    // 5. Airlines and their customers love it when travel is 2 or less hops. Submit nuts2.json and nuts2orLess.json.
    let nuts2 = nuts1.compose(&nuts1);
    let nuts2orless = nuts2.union(&nuts1);

    nuts2.to_file("./deliverables/nuts2.json");
    nuts2orless.to_file("./deliverables/nuts2orless.json");

    // 6. They try hard to get you from any city to any city in 3 hops. Submit nuts3.json and nuts3orLess.json.
    let nuts3 = nuts2.compose(&nuts2);
    let nuts3orless = nuts3.union(&nuts2);

    nuts3.to_file("./deliverables/nuts3.json");
    nuts3orless.to_file("./deliverables/nuts3orless.json");

    // 7. Barely acceptable as the worst case is 4 hops. Submit nuts4.json and nuts4orLess.
    let nuts4 = nuts3.compose(&nuts3);
    let nuts4orless = nuts4.union(&nuts3);

    nuts4.to_file("./deliverables/nuts4.json");
    nuts4orless.to_file("./deliverables/nuts4orless.json");

    // 8. What are all the connected cities, regardless of how many hops? Submit nutsT.json.
    let nuts_t = nuts1.trans_clos();
    nuts_t.to_file("./deliverables/nutsT.json");
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

        dbg!(m.trans_clos()); // TODO needs an assert
    }
}
