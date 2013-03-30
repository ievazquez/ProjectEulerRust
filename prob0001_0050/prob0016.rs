use core::num::{ One };

use std::bigint::{ BigInt };

use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 16,
    answer: "1366",
    solver: solve
};

fn solve() -> ~str {
    let mut i = One::one::<BigInt>();
    for uint::range(0, 1000) |_n| {
        i = i * BigInt::from_uint(2);
    }
    let sum = do str::byte_slice(i.to_str()) |buf| {
        buf.map(|c| *c - ('0' as u8)).foldl(0, |s, e| *s + (*e as uint))
    };
    return sum.to_str();
}
