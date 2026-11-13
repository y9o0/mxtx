#[macro_export]
macro_rules! eval {
    ($expr:ident, $($ident:ident = $value:expr), *) => {
        {
            let mut hashmap: ::std::collections::HashMap<String, f64> =
                    ::std::collections::HashMap::new();

            $( hashmap.insert(stringify!($ident).to_string(), $value); )*

            let ans: &$crate::Answer = &$expr;

            let $crate::Answer::Eval(expr) = ans else { panic!("expected Answer::Eval") };

            let rep_expr: $crate::Expr = expr.rep(&mut hashmap).unwrap();
            let result = $crate::Expr::codegen(&rep_expr).unwrap();

            match result {
                $crate::Expr::Number(x) => $crate::Answer::Num(x),
                $crate::Expr::Bool(x) => $crate::Answer::Bool(x),
                _ => {
                    panic!("unexpected result type: '{:?}'", result)
                }
            }
        }
    }
}

#[macro_export]
macro_rules! equation {
    ($name:ident ( $($p:ident),* ) = $eq:expr) => {
        fn $name($ ($p: f64),* ) -> $crate::Answer {
            let eq : &str = $eq;
            let eq = format!("IaF {}", eq);

            let ans = $crate::Parser::new(eq.to_string()).equation().unwrap();

            let result = $crate::eval!(ans, $($p = $p),*);
            result
        }
    };

    ($eq:expr) => {
        {
            let eq : &str = $eq;
            $crate::Parser::new(eq.to_string()).equation().unwrap()
        }
    }
}
