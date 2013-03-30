use core::util::{ unreachable };

use common::prime::{ Prime, num_of_divisors };
use common::calc::{ each_triangles };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 12,
    answer: "76576500",
    solver: solve
};

fn solve() -> ~str {
    let mut primes = Prime::new();

    for each_triangles |t| {
        let num = num_of_divisors(t, &mut primes);
        if num > 500 {
            return t.to_str();
        }
    }

    unreachable();
}