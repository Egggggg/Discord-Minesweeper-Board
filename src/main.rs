use rand::seq::SliceRandom;
use std::iter::repeat;

#[derive(Copy, Clone, Eq, PartialEq)]
enum BoardState {
    Empty,
    Bomb,
    Count(u8),
}

impl Default for BoardState {
    fn default() -> Self {
        Empty
    }
}

use std::fmt::{Display, Error, Formatter};
use BoardState::*;

const DIGITS: [&'static str; 9] = ["1️⃣", "2️⃣", "3️⃣", "4️⃣", "5️⃣", "6️⃣", "7️⃣", "8️⃣", "9️⃣"];

impl BoardState{
    pub fn inc(&mut self){
        match self{
            Empty => *self = Count(1u8),
            Count(v) => *v += 1,
            _ => (),
        }
    }
}

impl Display for BoardState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self{
            Empty => f.write_str("||`▫️`||"),
            Count(v) => f.write_fmt(format_args!("||`{}`||", DIGITS[*v as usize])),
            Bomb => f.write_str("||`💣`||"),
        }
    }
}

const LEN: usize = 8;
const WIDTH: usize = 8;
const N_BOMBS: u8 = (WIDTH + LEN / 2) as u8;

struct Board {
    state: [[BoardState; LEN]; WIDTH],
}

impl Board {
    pub fn gen(num_bombs: u8) -> Self {
        let mut ret = Self {
            state: [[Empty; LEN]; WIDTH],
        };
        let mut indecies: Vec<(usize, usize)> =
            (0..WIDTH).flat_map(|n| repeat(n).zip(0..LEN)).collect();
        indecies.shuffle(&mut rand::thread_rng());
        for (x, y) in indecies.into_iter().take(num_bombs as usize) {
            ret.state[x][y] = Bomb;
            if x + 1 < WIDTH {
                ret.state[x + 1][y].inc();
                if y + 1 < LEN {
                    ret.state[x + 1][y + 1].inc();
                }
                if y > 0 {
                    ret.state[x + 1][y - 1].inc();
                }
            }
            if x > 0 {
                ret.state[x - 1][y].inc();
                if y + 1 < LEN {
                    ret.state[x - 1][y + 1].inc()
                }
                if y > 0 {
                    ret.state[x - 1][y - 1].inc()
                }
            }
            if y + 1 < LEN {
                ret.state[x][y + 1].inc();
            }
            if y > 0 {
                ret.state[x][y - 1].inc();
            }
        }
        ret
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for v in self.state.iter() {
            for s in v.iter() {
                s.fmt(f)?;
                f.write_str(" ")?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn main() {
    println!("{}",Board::gen(N_BOMBS));
}
