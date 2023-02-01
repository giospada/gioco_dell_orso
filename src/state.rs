#[derive(Debug, Clone)]
pub struct State {
    pub hunter: [u32; 3],
    pub bear: u32,
}

impl State {
    pub fn from_hash_to_pos(mut hash: u64) -> Self {
        let mut res = Self {
            hunter: [0, 0, 0],
            bear: 0,
        };
        let mut i = 0;
        let mut current_hunter = 0;
        while hash > 0 {
            if hash % 3 == 1 {
                res.hunter[current_hunter] = i;
                current_hunter += 1;
            } else if hash % 3 == 2 {
                res.bear = i;
            }
            hash /= 3;
            i += 1;
        }
        res
    }
    pub fn from_pos_to_hash(hunter: [u32; 3], bear: u32) -> u64 {
        let mut res = 0;
        let base: u64 = 3;
        for i in hunter {
            res += base.pow(i);
        }
        res += base.pow(bear) * 2;
        res
    }

    pub fn to_hash(&self) -> u64 {
        Self::from_pos_to_hash(self.hunter, self.bear)
    }
    pub fn display(&self) {
        display_position(self.hunter, self.bear);
    }
}

#[test]
fn hash_test() {
    let end_states = [
        41, 775, 2127231, 46767537, 1250480673, 5983494219, 8652390921, 4395548511, 396995175,
        4793985, 8913, 115,
    ];
    let State { hunter, bear } = State::from_hash_to_pos(end_states[0]);
    assert!(hunter == [1, 2, 3]);
    assert!(bear == 0);
}
fn display_position(hunter: [u32; 3], bear: u32) {
    let mut cells = [' '; 21];
    for i in hunter {
        cells[i as usize] = 'h';
    }
    cells[bear as usize] = 'b';

    println!(
        "{}{}{}{}{}{}",
        "            ", cells[0], "            ", "             ", "0", "            "
    );
    println!(
        "{}{}{}{}{}{}{}{}{}{}",
        "        ",
        cells[1],
        "       ",
        cells[2],
        "        ",
        "         ",
        "1",
        "       ",
        "2",
        "        "
    );
    println!(
        "{}{}{}{}{}{}",
        "            ", cells[3], "            ", "             ", "3", "            "
    );
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        "  ",
        cells[4],
        "         ",
        cells[5],
        "         ",
        cells[6],
        "  ",
        "   ",
        "4",
        "         ",
        "5",
        "         ",
        "6",
        "  "
    );
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        "",
        cells[7],
        "   ",
        cells[8],
        "   ",
        cells[9],
        "   ",
        cells[10],
        "   ",
        cells[11],
        "   ",
        cells[12],
        "   ",
        cells[13],
        "",
        " ",
        "7",
        "   ",
        "8",
        "   ",
        "9",
        "  ",
        "10",
        "  ",
        "11",
        "  ",
        "12",
        "  ",
        "13",
        ""
    );
    println!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        "  ",
        cells[14],
        "         ",
        cells[15],
        "         ",
        cells[16],
        "  ",
        "  ",
        "14",
        "        ",
        "15",
        "        ",
        "16",
        ""
    );
    println!(
        "{}{}{}{}{}{}",
        "            ", cells[17], "            ", "            ", "17", "            "
    );
    println!(
        "{}{}{}{}{}{}{}{}{}{}",
        "        ",
        cells[18],
        "       ",
        cells[19],
        "        ",
        "        ",
        "18",
        "      ",
        "19",
        "        "
    );
    println!(
        "{}{}{}{}{}{}",
        "            ", cells[20], "            ", "            ", "20", "            "
    );
}
