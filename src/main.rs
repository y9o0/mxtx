use mxtx::parser::{equation, function};

fn main() -> anyhow::Result<()> {
    let mut f = function("f(x) = -10 + x - x - 10 / √(1 * 10 ^ 2)")?;
    let res = f.eval(&[1.0])?;
    let eq_res = equation("(10 + 1^9) / sqrt 10")?;

    println!("{} And {}", eq_res, res);
    Ok(())
}
