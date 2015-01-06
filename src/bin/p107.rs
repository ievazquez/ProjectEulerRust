//! [Problem 107](https://projecteuler.net/problem=107) solver.

#![warn(bad_style,
        unused, unused_extern_crates, unused_import_braces,
        unused_qualifications, unused_results, unused_typecasts)]

#![feature(phase)]

#[phase(plugin, link)] extern crate common;
extern crate "union-find" as union_find;

use std::io::{BufferedReader, File, IoResult};
use union_find::UnionFind;

fn compute<R: Reader>(r: R, size: uint) -> IoResult<uint> {
    let mut br = BufferedReader::new(r);

    let mut verts = Vec::new();
    for (i, line) in br.lines().enumerate() {
        let line = try!(line);
        for (j, s) in line.trim().split(',').enumerate() {
            if j <= i { continue }
            if let Some(n) = s.parse::<uint>() {
                verts.push(((i, j), n));
            }
        }
    }
    verts.sort_by(|a, b| a.1.cmp(&b.1));

    let mut uf = UnionFind::new(size);

    let mut saving = 0;
    for &((i, j), n) in verts.iter() {
        if uf.find(i, j) {
            saving += n;
        } else {
            uf.union(i, j);
        }
    }

    Ok(saving)
}

fn solve(file: File) -> IoResult<String> {
    Ok(try!(compute(file, 40)).to_string())
}

problem!("259679", "p107_network.txt", solve);

#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        let matrix = "-,16,12,21,-,-,-
16,-,-,17,20,-,-
12,-,-,28,-,31,-
21,17,28,-,18,19,23
-,20,-,18,-,-,11
-,-,31,19,-,-,27
-,-,-,23,11,27,-";

        assert_eq!(150, super::compute(matrix.as_bytes(), 7).unwrap());
    }
}
