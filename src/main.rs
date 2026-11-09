use mxtx::{equation, eval, sema::core::Answer};

fn main() {
    let eq: Answer = equation("x^2 - 61y^2 = 1");

    for y in 0..=100 {
        let x = y + 1;

        let result = eval!(eq, x = x as f64, y = y as f64);

        if result.get::<bool>().is_some_and(|r| r == true) {
            println!("({},{}) are a solution for the Pell equation ", x, y)
        }
    }
}
