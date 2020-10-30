// type _ value =
//   | Bool : bool -> bool value
//   | Int : int -> int value

// type _ expr =
//   | Value : 'a value -> 'a expr
//   | If : bool expr * 'a expr * 'a expr -> 'a expr
//   | Eq : 'a expr * 'a expr -> bool expr
//   | Lt : int expr * int expr -> bool expr

#[macro_use]
extern crate ad_astra_runtime;

type FragmentId = u32;
type BindId = u32;

pub enum Value {
    Bool(bool),
    Int(isize),
    Str(&'static str),
}

pub enum Step {
    Value(Value),
    IfExpr,
    EqExpr,
    LtExpr,
    Trace(usize),
}

ast! {
    Neighborhood, Path, Step, (for<T> Expr<T>) =>
        ((Expr<bool>) ::=
            (@m Step::Value(Value::Bool(_))) |
            (EqExpr (for<T> ((Expr<T>) ^ (Expr<T>)))) |
            (LtExpr ((Expr<isize>) ^ (Expr<isize>))));
        ((Expr<isize>) ::=
            (@m Step::Value(Value::Int(_))));
        ((Expr<&'static str>) ::=
            (@m Step::Value(Value::Str(_))));
        (for<T> ((Expr<T>) ::=
            (IfExpr ((Expr<bool>) ^ (Expr<T>) ^ (Expr<T>)))));
}

#[test]
fn test_simple_sexpr() {
    let tree = Neighborhood::with_paths(
        vec![
            path![Step::IfExpr, Step::Trace(0), Step::LtExpr, Step::Trace(0), Step::Value(Value::Int(420))],
            path![Step::IfExpr, Step::Trace(0), Step::LtExpr, Step::Trace(1), Step::Value(Value::Int(130))],
            path![Step::IfExpr, Step::Trace(1), Step::Value(Value::Str("a"))],
            path![Step::IfExpr, Step::Trace(2), Step::Value(Value::Str("b"))],
        ]
    );
    tree.validate();
}
