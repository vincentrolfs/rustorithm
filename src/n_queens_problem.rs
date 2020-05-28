// https://www.techiedelight.com/print-possible-solutions-n-queens-problem/

#[derive(Clone)]
struct Solution {
    positions: Vec<(u32, u32)>
}

fn populate_solutions(amount_rows: u32, amount_cols: u32, solutions: &mut Vec<Solution>){
    if amount_rows == 0 || amount_cols == 0 {
        return push_empty_solution(solutions);
    }
    populate_solutions(amount_rows - 1, amount_cols, solutions);
    add_queen(amount_rows - 1, amount_cols, solutions);
}

fn push_empty_solution(solutions: &mut Vec<Solution>){
    let empty_solution = Solution {
        positions: vec!()
    };
    solutions.push(empty_solution);
}

fn add_queen(row_index: u32, amount_cols: u32, solutions: &mut Vec<Solution>){
    let mut new_solutions = vec!();

    for solution in solutions.iter() {
        for col_index in 0..amount_cols {
            if ! has_conflict(row_index, col_index, &solution) {
                let mut new_solution = solution.clone();
                new_solution.positions.push((row_index, col_index));
                new_solutions.push(new_solution);
            }
        }
    }

    *solutions = new_solutions;
}

fn has_conflict(row_index: u32, col_index: u32, solution: &Solution) -> bool {
    for (row_taken, col_taken) in solution.positions.iter() {
        if  *row_taken == row_index ||
            *col_taken == col_index ||
            *row_taken + *col_taken == row_index + col_index ||
            (*row_taken as i32) - (*col_taken as i32) == (row_index as i32) - (col_index as i32)
        {
            return true;
        }
    }

    return false;
}

fn print_solutions(solutions: Vec<Solution>){
    println!("Number of solutions: {}", solutions.len());
    for solution in solutions.iter() {
        println!("--------------------");
        print_one_solution(&solution.positions);
    }
}

fn print_one_solution(positions: &Vec<(u32, u32)>){
    let len = positions.len() as u32;

    for row_index in 0..len {
        for col_index in 0..len {
            if positions.contains(&(row_index, col_index)) {
                print!("X");
            } else {
                print!("O");
            }
        }
        println!();
    }
}

pub fn main() {
    let mut solutions = vec!();
    populate_solutions(8, 8, &mut solutions);
    print_solutions(solutions);
}
