use core::hashmap::linear::{ LinearSet };

use common::prime::{ Prime, Factors };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 29,
    answer: "9183",
    solver: solve
};

fn solve() -> ~str {
    let mut ps  = Prime::new();
    let mut set = LinearSet::new();

    for uint::range(2, 101) |a| {
        let fs = iter::to_vec(&Factors::new(a, &mut ps));
        for uint::range(2, 101) |b| {
            set.insert(fs.map(|&(base, exp)| { (base, (exp as uint) * b) }));
        }
    }

    return set.len().to_str();
}