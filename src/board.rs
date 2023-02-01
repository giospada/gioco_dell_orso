use crate::state::State;

pub struct Board {
    pub adj: [Vec<u32>; 21],
}

impl Board {
    pub fn new() -> Self {
        Self {
            adj: Self::get_adj_list(),
        }
    }
    fn get_adj_list() -> [Vec<u32>; 21] {
        [
            vec![1, 2, 3],
            vec![0, 3, 4],
            vec![0, 3, 6], // 2
            vec![0, 1, 2, 5],
            vec![1, 7, 8], // 4
            vec![3, 9, 10, 11],
            vec![2, 12, 13], // 6
            vec![4, 8, 14],
            vec![7, 4, 14, 9], // 8
            vec![8, 10, 5, 15],
            vec![5, 9, 11, 15], // 10
            vec![5, 10, 15, 12],
            vec![11, 6, 16, 13], // 12
            vec![6, 12, 16],
            vec![7, 8, 18], // 14
            vec![9, 10, 11, 17],
            vec![12, 13, 19], // 16
            vec![15, 18, 19, 20],
            vec![14, 17, 20], // 18
            vec![16, 17, 20],
            vec![18, 17, 19],
        ]
    }
    pub fn reaceble(&self, hash: u64, bear_turn: bool) -> Vec<u64> {
        let State { hunter, bear } = State::from_hash_to_pos(hash);
        let mut res = vec![];
        if !bear_turn {
            let mut hunter_copy = hunter.clone();
            for (i, pos) in hunter.iter().enumerate() {
                for next in self.adj[*pos as usize].iter() {
                    if *next == hunter[0]
                        || *next == hunter[1]
                        || *next == hunter[2]
                        || *next == bear
                    {
                        continue;
                    }
                    hunter_copy[i] = *next;
                    res.push(State::from_pos_to_hash(hunter_copy, bear));
                    hunter_copy[i] = hunter[i];
                }
            }
        } else {
            for next in self.adj[bear as usize].iter() {
                if *next == hunter[0] || *next == hunter[1] || *next == hunter[2] {
                    continue;
                }
                res.push(State::from_pos_to_hash(hunter, *next));
            }
        }
        res
    }
}
#[test]
fn reaceble_test() {
    let b = Board::new();
    b.reaceble(41, true);
}
