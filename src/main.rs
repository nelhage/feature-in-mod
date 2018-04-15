mod ast;
mod transform;

use ast::AST;

fn main() {
    let ast = AST::Add(
        Box::new(AST::Var("x".to_string())),
        Box::new(AST::Add(
            Box::new(AST::Add(
                Box::new(AST::Integer(0)),
                Box::new(AST::Integer(1)),
            )),
            Box::new(AST::Integer(2)),
        )),
    );

    println!("before={:?}", &ast);
    println!("after={:?}", transform::reassociate(ast));
}
