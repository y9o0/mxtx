use mxtx::{equation, sema::core::Answer};

fn main() {
    let result: Answer = equation("(x! · φ ÷ 700.2130) mod e where x = 10");
    println!("{}", result)
}
