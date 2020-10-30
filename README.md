## ad-astra •

♫ Every *turn* I take, every *trail* I track ♫
♫ Every *path* I make, every *road* leads back ♫
♫ To the place I know, where I cannot go, where I long to be ♫

**ad-astra** is a tool for working with Abstract Syntax Trees in your program.

The main goal is to describe any AST in an intuitive and precise manner. In some languages, means for such a description are achieved with *(Generalized) Algebraic Data Types*. Such means are best utilized with functional flavor in the language, so they're typically found in functional languages.

In ad-astra, we experiment with language-oriented programming. We take a step back to a simplistic and inefficient format for ASTs, then take a larger step forward towards richer AST formulation through context-free grammars. For this purpose, the context-free grammars we use are supplemented with a new operator, the offshoot operator (`^`).

### Progress

**ad-astra** is at the stage of little more than a concept. It accepts restricted CFGs through kludgy syntax. More is yet to come as the [Panini](https://github.com/pczarn/panini) project progresses, because the progress of both projects may go in lockstep.

### Examples

#### Simple s-expression definition

##### OCaml & GADT

```ocaml
type _ value =
    | Bool : bool -> bool value
    | Int : int -> int value

type _ expr =
    | Value : 'a value -> 'a expr
    | If : bool expr * 'a expr * 'a expr -> 'a expr
    | Eq : 'a expr * 'a expr -> bool expr
    | Lt : int expr * int expr -> bool expr
```

##### Haskell & GADT

*TODO*

##### Agda & GADT

```agda
open import Agda.Builtin.Bool
open import Agda.Builtin.Int

data Value : Set → Set where
  bool : Bool → Value Bool
  int : Int → Value Int

data Expr : Set → Set where
  value : {n : Set} → Value n → Expr n
  if_expr : {n : Set} → Expr Bool → Expr n → Expr n → Expr n
  eq_expr : {n : Set} → Expr n → Expr n → Expr n
  lt_expr : Expr Int → Expr Int → Expr Bool
```

#### Rust & ADT

```rust
enum Value {
    Bool(bool),
    Isize(isize),
}

enum Expr {
    Value(Value),
    IfExpr {
        condition: Box<Expr>,
        ifthen: Box<Expr>,
        elsethen: Box<Expr>,
    },
    EqExpr(Box<Expr>, Box<Expr>),
    LtExpr(Box<Expr>, Box<Expr>)
}
```

##### Rust & GADT with ad-astra

###### Explicit code, compile-time
```rust
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
            (Expr<bool> ::=
                Value(Value::Bool(_)) |
                EqExpr for<T> (Expr<T> ^ Expr<T>) |
                LtExpr (Expr<isize> ^ Expr<isize>)
            )
            (Expr<isize> ::=
                Value(Value::Int(_))
            )
            (for<T> Expr<T> ::=
                IfExpr (Expr<bool> ^ Expr<T> ^ Expr<T>)
            )
            (@allow for<T> Expr<T>)
    }
}
```

###### Implicit definitions

```rust
#[ast]
mod ast {
    // ...

    pub struct Neighborhood {
        #[ast_paths]
        paths: Vec<Path>,
    }

    pub struct Path {
        #[ast_steps]
        steps: Vec<Step>,
    }

    // ...
}
```

###### Code, run-time

```rust
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

ast! {
    Neighborhood, Path, Step, (for<T> Expr<T>) =>
        ((Expr<bool>) ::=
            (Value(Value::Bool(_))) |
            (EqExpr (for<T> (Expr<T> ^ Expr<T>))) |
            (LtExpr (Expr<isize> ^ Expr<isize>))
        )
        ((Expr<isize>) ::=
            (Value(Value::Int(_)))
        )
        ((for<T> (Expr<T>)) ::=
            (IfExpr (Expr<bool> ^ Expr<T> ^ Expr<T>))
        )
}
```

#### Simple s-expression use

*TODO*

### Glossary

* **Neighborhood** — a family of Abstract Syntax Trees.
* **Path** — a single road through the *Neighborhood*, which includes the root of the tree, a leaf of the tree and every step in between.
* **Step** — an element of a *Path*, which carries meaning.
* **Trace** — a carrier of an index, which supplements ordering within collections of *Path*s.

### License

Dual-licensed for compatibility with the Rust project.

Licensed under the Apache License Version 2.0:
http://www.apache.org/licenses/LICENSE-2.0, or the MIT license:
http://opensource.org/licenses/MIT, at your option.
