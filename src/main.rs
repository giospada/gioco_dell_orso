use std::collections::{BTreeMap, BTreeSet, VecDeque};
mod state;

mod board;

use board::Board;
use state::State;

const HUNTER_IND: usize = 0;
const BEAR_IND: usize = 1;

fn main() {
    let stdin = std::io::stdin();

    let mut sim = Simulation::start_state();
    sim.run_simulation();

    let mut current_state = State {
        hunter: [0, 1, 2],
        bear: 20,
    };
    //sto scrivendo un prompt per giocarci interattivamente conro l'orso
    let mut bear_trun = false;
    let b = Board::new();
    loop {
        current_state.display();
        let mut buf = String::new();
        println!("mosse disponibili:");
        let next_state = b.reaceble_state(current_state.clone(), bear_trun);
        let mut best = 0;
        let mut best_val = u32::MAX;
        for (index, hash) in next_state.iter().enumerate() {
            let value = sim.end_state[if !bear_trun { BEAR_IND } else { HUNTER_IND }][hash];
            if value < best_val {
                best_val = value;
                best = index;
            }
            println!(
                "{} : {:?}, hunter win in {} moves",
                index,
                State::from_hash_to_pos(*hash),
                value
            );
        }
        println!("scrivi il numero della mossa che vuoi fare o b per best (quit per uscire):");
        match stdin.read_line(&mut buf) {
            Err(_) => break,
            Ok(_) => {}
        }
        if buf.trim() == "quit" {
            break;
        }
        if buf.trim() == "b" {
            current_state = State::from_hash_to_pos(next_state[best]);
        } else {
            let ind = buf.trim().parse::<usize>().unwrap();
            current_state = State::from_hash_to_pos(next_state[ind]);
        }
        bear_trun = !bear_trun;
        println!("enter to continue");
    }
}

struct Simulation {
    unexplored_state: [BTreeSet<u64>; 2],
    end_state: [BTreeMap<u64, u32>; 2],
    board: Board,
    dist: u32,
}

impl Simulation {
    fn start_state() -> Self {
        let mut res = Self {
            unexplored_state: [Self::get_all_state(), Self::get_all_state()],
            end_state: [BTreeMap::new(), Self::get_end_states()],
            board: Board::new(),
            dist: 0,
        };
        for (key, _) in res.end_state[BEAR_IND].iter() {
            res.unexplored_state[BEAR_IND].remove(key);
        }
        res
    }
    fn get_end_states() -> BTreeMap<u64, u32> {
        let mut res = BTreeMap::new();
        let end_state = [
            41, 775, 2127231, 46767537, 1250480673, 5983494219, 8652390921, 4395548511, 396995175,
            4793985, 8913, 115,
        ];

        for i in end_state.iter() {
            res.insert(*i, 0);
        }

        res
    }

    fn get_all_state() -> BTreeSet<u64> {
        let mut res = BTreeSet::new();
        for i in 0..=(20 - 2) {
            for j in i + 1..=(20 - 1) {
                for z in j + 1..=(20) {
                    for bear in 0..=20 {
                        if bear == i || bear == j || bear == z {
                            continue;
                        }
                        res.insert(State::from_pos_to_hash([i, j, z], bear));
                    }
                }
            }
        }
        res
    }

    fn run_simulation(&mut self) {
        let mut any_new_state = true;
        while any_new_state {
            self.dist += 1;
            any_new_state = self.find_new_end_state();
        }
    }
    fn find_new_end_state(&mut self) -> bool {
        if self.dist % 2 == 1 {
            self.cerca_stati_vittoria_cacciatore()
        } else {
            self.cerca_stati_perdita_orso()
        }
    }
    fn cerca_stati_perdita_orso(&mut self) -> bool {
        let mut added = VecDeque::new();
        for unexplored in self.unexplored_state[BEAR_IND].iter() {
            if self
                .board
                .reaceble(*unexplored, true)
                .iter()
                .all(|x| self.end_state[HUNTER_IND].contains_key(x))
            {
                added.push_back(*unexplored);
            }
        }
        let res = !added.is_empty();
        while let Some(curr) = added.pop_front() {
            self.unexplored_state[BEAR_IND].remove(&curr);
            self.end_state[BEAR_IND].insert(curr, self.dist);
        }
        res
    }
    fn cerca_stati_vittoria_cacciatore(&mut self) -> bool {
        let mut added = VecDeque::new();
        for unexplored in self.unexplored_state[HUNTER_IND].iter() {
            if self
                .board
                .reaceble(*unexplored, false)
                .iter()
                .any(|x| self.end_state[BEAR_IND].contains_key(x))
            {
                added.push_back(*unexplored);
            }
        }
        let res = !added.is_empty();
        while let Some(curr) = added.pop_front() {
            self.unexplored_state[HUNTER_IND].remove(&curr);
            self.end_state[HUNTER_IND].insert(curr, self.dist);
        }
        res
    }
}
//
