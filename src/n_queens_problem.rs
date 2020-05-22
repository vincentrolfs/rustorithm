// https://www.techiedelight.com/print-possible-solutions-n-queens-problem/

use std::collections::HashSet;

const N: i32 = 4;

struct QueensData {
    positions: Vec<(i32, i32)>,
    i_blocked: HashSet<i32>,
    j_blocked: HashSet<i32>,
}

fn add_queen(index: usize, i: i32, j: i32, queens: &mut QueensData) {
    if queens.positions.len() <= index {
        queens.positions.push((i, j));
    } else {
        queens.positions[index] = (i, j);
    }
    queens.i_blocked.insert(i);
    queens.j_blocked.insert(j);
}

fn remove_queen(i: i32, j: i32, queens: &mut QueensData) {
    queens.i_blocked.remove(&i);
    queens.j_blocked.remove(&j);
}

fn populate_queens(index: usize, mut queens: &mut QueensData) {
    if index == N as usize {
        println!("{:?}\n", queens.positions);
        return;
    }

    for i in 0..N {
        if queens.i_blocked.contains(&i) {
            continue;
        }

        for j in 0..N {
            if ! queens.j_blocked.contains(&j) {
                add_queen(index, i, j, &mut queens);
                populate_queens(index + 1, queens);
                remove_queen(i, j, &mut queens);
            }
        }
    }
}

pub fn main() {
    let mut queens = QueensData {
        positions: vec!(),
        i_blocked: HashSet::new(),
        j_blocked: HashSet::new()
    };

    populate_queens(0, &mut queens);
}
