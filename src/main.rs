use mxtx::equation;

fn main() {
    let result = equation("(8 + 4) + tan(cos(sin(sqrt (pi * 3 - 2 * e))))").as_num();
    println!("{}", result);
}
