// type _ value =
//   | Bool : bool -> bool value
//   | Int : int -> int value

// type _ expr =
//   | Value : 'a value -> 'a expr
//   | If : bool expr * 'a expr * 'a expr -> 'a expr
//   | Eq : 'a expr * 'a expr -> bool expr
//   | Lt : int expr * int expr -> bool expr

extern crate enum_ast;

use self::ast::*;

type FragmentId = u32;
type BindId = u32;

#[ast]
mod ast {
    pub enum Value {
        Bool(bool),
        Int(isize),
    }

    pub enum Step {
        Value(Value),
        IfExpr,
        EqExpr,
        LtExpr,
        #[trace]
        Trace(usize),
    }

    neighborhood! {
        Neighborhood, Path, Step =>
            (@stmt Expr<bool> ::=
                Value(Value::Bool(_)) |
                EqExpr for<T> (Expr<T> ^ Expr<T>) |
                LtExpr (Expr<isize> ^ Expr<isize>)
            )
            (@stmt Expr<isize> ::=
                Value(Value::Int(_))
            )
            (@stmt for<T> Expr<T> ::=
                IfExpr (Expr<bool> ^ Expr<T> ^ Expr<T>)
            )
            (@allow for<T> Expr<T>)
    }
}

#[test]
fn test_simple_sexpr() {
    let tree = InputNeighborhood {
        paths: vec![
            path![Step::IfExpr, Step::Trace(0), Step::LtExpr, Step::Trace(0), Step::Value(Value::Int(420))],
            path![Step::IfExpr, Step::Trace(0), Step::LtExpr, Step::Trace(1), Step::Value(Value::Int(130))],
            path![Step::IfExpr, Step::Trace(1), Step::Value(0, "a")],
            path![Step::IfExpr, Step::Trace(2), Step::Value(1, "b")],
        ]
    };
    tree.validate();
}
