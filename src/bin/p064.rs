//! [Problem 64](https://projecteuler.net/problem=64) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate cont_frac;
extern crate num;

use num::Integer;

fn solve() -> String {
    range(1, 10001)
        .map(cont_frac::sqrt)
        .map(|(_a0, an)| an.len())
        .filter(|an| an.is_odd())
        .count()
        .to_string()
}

problem!("1322", solve);
