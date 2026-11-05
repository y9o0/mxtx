use mxtx::{equation, sema::core::Answer};

fn main() {
    let result: Answer = equation("(8 + 4) + tan(cos(sin(sqrt (pi * 3 - 2 * e)))) < 1^9");
    println!("{}", result);
}
