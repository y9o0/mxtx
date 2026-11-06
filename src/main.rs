use mxtx::{equation, sema::core::Answer};

fn main() {
    let result: Answer = equation("ln(log_3(|(-8 + 4) + tan(cos(sin(sqrt (pi * 3 - 2 * e))))|))");
    println!("{}", result);
}
