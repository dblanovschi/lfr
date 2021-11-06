use lfr_parser::lfr_syntax::ast::{
    AstNode,
    Root,
};
use lfr_parser::lfr_syntax::SyntaxNode;

mod panic_handler;

fn main()
{
    panic_handler::init();

    let (node, errors) = lfr_parser::parser::parse(
                                                   r#"
        import 'a'
        import a
        if 0 == 1 {
            print(1)
        } else {
            print(0)
        }
        
        x = 1
        +2

        return 'abc'
            .len()
        
        /*
        Hello
        */
        "#,
    );
    dbg!(&errors);

    let node = SyntaxNode::new_root(node);
    // lfr_parser::parser::print_ast(&node, 0);
    let root = Root::cast(node).unwrap();

    for import_stmt in root.import_stmts() {
        lfr_parser::parser::print_ast(import_stmt.import_target()
                                                 .unwrap()
                                                 .syntax(),
                                      0);
    }

    // for stmt in root.stmts() {
    //     lfr_parser::parser::print_ast(stmt.syntax(), 0);
    // }
}
