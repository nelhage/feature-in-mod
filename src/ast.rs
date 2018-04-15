#[derive(Debug)]
pub enum AST {
    Integer(i64),
    Add(Box<AST>, Box<AST>),
    Var(String),
}
