#[link(name = "prob0047", vers = "0.0")];
#[crate_type = "lib"];

extern mod common;

use std::util;
use std::iterator::{ Counter, IteratorUtil };
use common::prime;
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 47,
    answer: "134043",
    solver: solve
};

pub fn solve() -> ~str {
    let len = 4;
    let num_factor = 4;

    let mut cnt = 0;
    let mut it = Counter::new::<uint>(1, 1);
    for it.advance |n| {
        if prime::factorize(n).count() != num_factor {
            cnt = 0;
            loop;
        }

        cnt += 1;
        if cnt == len {
            return (n + 1 - len).to_str();
        }
    }

    util::unreachable();
}
