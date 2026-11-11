use mxtx::equation;

fn main() {
    equation!(f(x) = "2^3^2 * x");
    println!("{}", f(1.0));
}
