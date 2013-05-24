extern mod extra;
extern mod common;

priv use extra::time;
priv use common::problem::{ Problem };

priv mod problem;

priv fn each_problems(f: &fn(&Problem) -> bool) -> bool {
    for problem::problems.each |&p| {
        if !f(p) { return false; }
    }
    return true;
}

priv static NSEC_PER_SEC: u64 = 1000000000;
priv fn nanosec_to_str(nsec: u64) -> ~str {
    return fmt!("%3u.%09u",
         (nsec / NSEC_PER_SEC) as uint,
         (nsec % NSEC_PER_SEC) as uint);
}

priv fn solve(p: &Problem) -> u64 {
    let start_time = time::precise_time_ns();
    let comp_answer = (p.solver)();
    let calc_time   = time::precise_time_ns() - start_time;

    io::println(fmt!("%-5u %-20s %s",
                     p.id, comp_answer, nanosec_to_str(calc_time)));

    assert_eq!(comp_answer, p.answer.to_str());

    return calc_time;
}

fn main() {
    let nums = os::args().filter_mapped(|&s| uint::from_str(s));

    let mut total_time = 0;
    let mut solve_cnt = 0;
    if nums.is_empty() {
        for each_problems |p| {
            total_time += solve(p);
            solve_cnt += 1;
        }
    } else {
        for nums.each |&n| {
            for each_problems |p| {
                if p.id == n {
                    total_time += solve(p);
                    solve_cnt += 1;
                }
            }
        }
    }

    if solve_cnt > 1 {
        io::println(fmt!("TOTAL %-12s %s", "", nanosec_to_str(total_time)));
    }
}
