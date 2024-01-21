#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(Value),
}
