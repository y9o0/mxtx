use mxtx::parser::{equation, function};

fn main() -> anyhow::Result<()> {
    let mut f = function("f(x) = pi * (10 + x - x - 10 / √(1 * 10 ^ 2))")?;
    let res = f.eval(&[1.0])?;
    let eq_res = equation("π / 2")?;

    println!("{} And {}", eq_res, res);
    Ok(())
}
