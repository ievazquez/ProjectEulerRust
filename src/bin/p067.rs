//! [Problem 67](https://projecteuler.net/problem=67) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;

use std::cmp;
use std::io::{File, IoResult, BufferedReader};

fn solve(file: File) -> IoResult<String> {
    let mut input = BufferedReader::new(file);

    let mut triangle = input.lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .map(|line| line.words().filter_map(StrExt::parse).collect::<Vec<uint>>())
        .collect::<Vec<_>>();

    let last = triangle.pop().unwrap();
    let ans = triangle.iter()
        .rev()
        .fold(last, |prev, elem| {
            Vec::from_fn(elem.len(), |i| elem[i] + cmp::max(prev[i], prev[i + 1]))
        })[0];

    Ok(ans.to_string())
}

problem!("7273", "p067_triangle.txt", solve);
