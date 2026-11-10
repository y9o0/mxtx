use mxtx::equation;

fn main() {
    equation!(f(x, y) = "x^2 - y = 1");
    let result = f(10.0, 9.0);
    let result2 = equation!("1 * 10");

    println!("{}", result);
    println!("{}", result2);
}
