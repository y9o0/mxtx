pub mod definitions;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    #[test]
    fn test_lexer() {
        let tokens = tokenize("f(x) = x").unwrap();
        println!("{:?}", tokens)
    }
}

#[cfg(test)]
mod test_parser {
    use crate::parser::function;

    #[test]
    fn test_parser() -> anyhow::Result<()> {
        function("f(x, y, z, w)")
    }
}
