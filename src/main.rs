use mxtx::equation;

fn main() {
    equation!(f(x) = "2^3^2 * x domain: 5 <= x <= 10 and x != 9 range:  10000 > out > 2 ");
    println!("{}", f(10.0));
}
