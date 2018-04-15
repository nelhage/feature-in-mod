#![feature(box_patterns)]

use ast::AST;

pub fn reassociate(ast: AST) -> AST {
    match ast {
        AST::Add(box AST::Add(ll, lr), r) => AST::Add(ll, Box::new(AST::Add(lr, r))),
        AST::Add(l, r) => AST::Add(Box::new(reassociate(*l)), Box::new(reassociate(*r))),
        _ => ast
    }
}
