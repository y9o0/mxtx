use mxtx::scanner::core::Scanner;

fn main() -> anyhow::Result<()> {
    let some_tok = Scanner::new(String::from("=,-")).scanner()?;
    println!("{:?}", some_tok);
    Ok(())
}
