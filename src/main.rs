use mxtx::equation;

fn main() {
    equation!(f(x) = "2^3^2 * x domain: x >= 10 range: out > 2 ");
    println!("{}", f(10.0));
}
