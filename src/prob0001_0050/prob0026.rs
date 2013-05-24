#[link(name = "prob0026", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use common::extiter::{ ExtIteratorUtil, Range };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 26,
    answer: "983",
    solver: solve
};

fn get_cycle_len(n: uint) -> uint {
    if n == 1 { return 1; }
    let mut buf = vec::from_elem(n, None);
    let mut rem = 1;
    let mut idx = 1;
    loop {
        let new_rem = rem % n;
        match buf[new_rem] {
            Some(i) => { return idx - i; }
            None    => { buf[new_rem] = Some(idx); }
        }
        idx += 1;
        rem = new_rem * 10;
    }
}

pub fn solve() -> ~str {
    return Range::new::<uint>(2, 1000)
        .max_as(|&n| get_cycle_len(n))
        .to_str();
}