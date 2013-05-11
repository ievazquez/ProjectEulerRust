use common::calc::{ each_prim_pythagorean };
use common::problem::{ Problem };

pub static problem: Problem<'static> = Problem {
    id: 86,
    answer: "1818",
    solver: solve
};

fn get_count(m: uint) -> uint {
    let mut cnt = 0u;
    for uint::range(0, m) |max_a| {
        for each_prim_pythagorean(max_a) |p, q, _| {
            for uint::range(1, m / q + 1) |k| {
                cnt += k * p / 2;
            }

            for uint::range(1, m / p + 1) |k| {
                let end = uint::min(k * p, k * q / 2) + 1;
                let start = k * q - k * p;
                if end > start { cnt += end - start; }
            }
        }
    }
    return cnt;
}

// cuboid: (a, b, c),  a <= b <= c <= M
// => S = sqrt(c^2 + (a + b)^2)
fn solve() -> ~str {
    let limit_cnt = 1000000;

    let mut lim = 1;
    let mut cnt = get_count(lim);
    while cnt < limit_cnt {
        lim *= 2;
        cnt = get_count(lim);
    }

    let mut m = 0;
    while lim != 0 {
        let ix = m + (lim >> 1);
        let cnt = get_count(ix);
        match cnt.cmp(&limit_cnt) {
            Equal => break,
            Less  => {
                m = ix + 1;
                lim -= 1;
            }
            Greater => {}
        }
        lim >>= 1;
    }

    return m.to_str();
}
