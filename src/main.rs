use mxtx::function;

fn main() {
    function!(f(x, y) = "x^2 - y = 1");
    let result = f(10.0, 9.0);

    println!("{}", result)
}
