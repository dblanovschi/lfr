use lfr_syntax::ast;

use crate::db::DefDatabase;

pub fn lower_root(root: &ast::Root, db: &dyn DefDatabase) {
    for stmt in root.stmts() {
        lower_stmt(&stmt, db);
    }
}

pub fn lower_stmt(stmt: &ast::Stmt, db: &dyn DefDatabase) {
    match stmt {
        ast::Stmt::ExprStmt(e) => {
            let lower = lower_expr(&e.expr().unwrap(), db);
        }
        ast::Stmt::DeclarationStmt(d) => todo!(),
        ast::Stmt::WhileStmt(_) => todo!(),
        ast::Stmt::ForStmt(_) => todo!(),
    }
}

pub fn lower_expr(expr: &ast::Expr, db: &dyn DefDatabase) {
    match expr {
        ast::Expr::PrimaryExpr(pe) => match pe.inner().unwrap() {
            ast::PrimaryExprInner::TupleExpr(_) => todo!(),
            ast::PrimaryExprInner::ArrExpr(_) => todo!(),
            ast::PrimaryExprInner::Block(_) => todo!(),
            ast::PrimaryExprInner::LitVal(lv) => {
                match lv.inner().unwrap() {
                    ast::LitValInner::StringLit(slit) => slit.str_token(),
                    ast::LitValInner::NumberLit(_) => todo!(),
                    ast::LitValInner::BooleanLit(_) => todo!(),
                };
            }
            ast::PrimaryExprInner::IfExpr(_) => todo!(),
            ast::PrimaryExprInner::BreakStmt(_) => todo!(),
            ast::PrimaryExprInner::ContinueStmt(_) => todo!(),
            ast::PrimaryExprInner::ReturnStmt(_) => todo!(),
            ast::PrimaryExprInner::Path(_) => todo!(),
        },
        ast::Expr::BinExpr(_) => todo!(),
        ast::Expr::PrefixUnaryExpr(_) => todo!(),
        ast::Expr::FnCallExpr(_) => todo!(),
        ast::Expr::IndexExpr(_) => todo!(),
        ast::Expr::MemberAccessExpr(_) => todo!(),
        ast::Expr::MethodCallExpr(_) => todo!(),
        ast::Expr::FnDef(_) => todo!(),
    }
}
