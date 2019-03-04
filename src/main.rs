mod problem_1;
mod problem_2;
mod problem_3;
mod problem_4;
mod problem_5;
mod problem_6;

fn main() {
    println!("Project Euler in Rust");

    let current_problem = 6;

    println!("Calculating problem {}", current_problem);
    let result = match current_problem {
        1 => problem_1::run(),
        2 => problem_2::run(),
        3 => problem_3::run(),
        4 => problem_4::run(),
        5 => problem_5::run(),
        6 => problem_6::run(),
        _ => {panic!("Invalid current problem!")},
    };
    println!("Result ~ {:?}", result);
}
