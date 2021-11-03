use parser::syntax::{SyntaxNode, ast::{AstNode, Root}};

fn main() {
    let node = parser::parser::parse(
        r#"
    if 0 == 1 {
        print(1)
    } else {
        print(0)
    }
    
    x = 1
    +2

    return 3
    "#,
    )
    .0;

    let node = SyntaxNode::new_root(node);
    // parser::parser::print_ast(&node, 0);
    let root = Root::cast(node).unwrap();

    for stmt in root.stmts() {
        parser::parser::print_ast(stmt.syntax(), 0);
    }
}
