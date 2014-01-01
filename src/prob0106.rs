#[crate_type = "rlib"];

extern mod math;
use std::iter::AdditiveIterator;
use std::hashmap::HashMap;
use prime = math::oldprime;

pub static EXPECTED_ANSWER: &'static str = "21384";

// n = 4
// (i, j): i: size of B, j: size of C
// (1,1) => 4C2 * (2C1 / 2) =  6
// (1,2) => 4C3 * 3C1       = 12
// (1,3) => 4C4 * 4C1       =  4
// (2,2) => 4C4 * (4C2 / 2) =  3
//                        ==> 25

// n = 7
// (1,1) => 7C2 * (2C1 / 2) = 21 * (2/2)  =  21
// (1,2) => 7C3 * 3C1       = 35 * 3      = 105
// (1,3) => 7C4 * 4C1       = 35 * 4      = 140
// (1,4) => 7C5 * 5C1       = 21 * 5      = 105
// (1,5) => 7C6 * 6C1       =  7 * 6      =  42
// (1,6) => 7C7 * 7C1       =  1 * 7      =   7
// (2,2) => 7C4 * (4C2/ 2)  = 35 * (6/2)  = 105
// (2,3) => 7C5 * 5C2       = 21 * 10     = 210
// (2,4) => 7C6 * 6C2       =  7 * 15     = 105
// (2,5) => 7C7 * 7C2       =  1 * 21     =  21
// (3,3) => 7C6 * (6C3 / 2) =  7 * (20/2) =  70
// (3,4) => 7C7 * 7C3       =  1 * 35     =  35
//                                      ==> 966

// 7 => (2,2)
// B B C C => S(B) < S(C)
// B C B C => S(B) < S(C)
// B C C B => ??
// 1 * 7C4 = 35

// 7 => (3,3)
// B B B C C C => S(B) < S(C)
// B B C B C C => S(B) < S(C)
// B B C C B C => S(B) < S(C)
// B B C C C B => ??
// B C B B C C => S(B) < S(C)
// B C B C B C => S(B) < S(C)
// B C B C C B => ??
// B C C B B C => ??
// B C C B C B => ??
// B C C C B B => ??
// 5 * 7C6 = 35

// f(i,j) := number of the pairs which satisfy S(B) < S(C)
// f(i,i) = f(i-1,i)
// f(i,j) if i < j = f(i,j-1) + f(i-1,j)
// f(i,j) if i > j = 0
// f(0,0) = 0
// f(i,0) = 0 if i > 0
// f(0,j) = 1 if j > 0

// f(1,1) = f(0,1)
//        = 1
// f(2,2) = f(1,2)
//        = f(0,2) + f(1,1)
//        = 1      + 1
//        = 2
// f(3,3) = f(2,3)
//        = f(2,2) + f(1,3)
//        = 2      + f(1,2) + f(0,3)
//        = 2      + 2      + 1
//        = 5

// N(A) = n
// N(B) = N(C) = k
// => nC2k * (2kCk/2 - f(k,k)) pairs
// answer: \sum_{k=1}^{n/2} (nC2k * (2kCk/2 - f(k,k)))

fn f(i: uint, j: uint, map: &mut HashMap<(uint, uint), uint>) -> uint {
    match (i, j) {
        (0, 0) => return 0,
        (_, 0) => return 0,
        (0, _) => return 1,
        _ if i == j => return f(i-1, j, map),
        _ if i >  j => return 0,
        _ => {}
    }

    match map.find(&(i, j)) {
        Some(n) => return *n,
        None => {}
    }

    let val = f(i, j - 1, map) + f(i - 1, j, map);
    map.insert((i, j), val);
    val
}

fn get_num_pairs(n: uint) -> uint {
    let mut map = HashMap::new();
    range(1, n / 2 + 1).map(|k| {
            prime::comb(n, 2*k) * (prime::comb(2 * k, k) / 2 - f(k, k, &mut map))
        }).sum()
}

pub fn solve() -> ~str {
    get_num_pairs(12).to_str()
}

#[cfg(test)]
mod test {
    use std::hashmap::HashMap;

    #[test]
    fn test_f() {
        let mut map = HashMap::new();
        assert_eq!(super::f(1, 1, &mut map), 1);
        assert_eq!(super::f(2, 2, &mut map), 2);
        assert_eq!(super::f(3, 3, &mut map), 5);
        assert_eq!(super::f(4, 4, &mut map), 14);
        assert_eq!(super::f(5, 5, &mut map), 42);
    }
    // f(4,4) = f(3,4)
    //        = f(3,3) + f(2,4)
    //        = 5      + f(2,3) + f(1,4)
    //        = 5      + 5      + f(1,3) + f(0,4)
    //        = 5      + 5      + 3      + 1
    //        = 14
    // f(5,5) = f(4,5)
    //        = f(4,4) + f(3,5)
    //        = 14     + f(3,4) + f(2,5)
    //        = 14     + 14     + f(2,4) + f(1,5)
    //        = 14     + 14     + 9      + f(1,4) + f(0,5)
    //        = 14     + 14     + 9      + 4      + 1
    //        = 42

    #[test]
    fn test_get_num_pairs() {
        assert_eq!(super::get_num_pairs(4), 1);
        assert_eq!(super::get_num_pairs(7), 70);
    }
}
