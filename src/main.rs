use mxtx::{equation, eval, sema::core::Answer};

fn main() {
    for i in 0..=100 {
        let eq: Answer = equation("x!");
        let result = eval!(eq, x = i as f64);

        println!("{}", result);
    }
}
