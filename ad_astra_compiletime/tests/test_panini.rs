extern crate enum_ast;

use self::ast::*;

type FragmentId = u32;
type BindId = u32;

#[ast]
mod ast {
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Step {
        Alternative(usize),
        Idx(usize),
        Fragment(FragmentId),
        StmtFragment(FragmentId),
        StmtIdx(usize),
        // Class(Class, Symbol),
        // RightQuote,
        Bind { bind_id: BindId, idx: usize },
        Sequence {
            min: u32,
            max: Option<u32>,
        },
        SequenceEnd,
        SequenceToken,
        #[upper_bound]
        Max,
    }

    neighborhood! {
        Neighborhood, Path, Step =>
            (common ::= Alternative | Idx | Bind | Sequence)
            (start ::= StmtFragment StmtIdx common* (Fragment | Alternative))
            (@allow start)
    }
}

#[test]
fn test_panini() {
    let tree = InputTree {
        paths: vec![
            path![Step::StmtFragment(0), Step::StmtIdx(0)],
            path![],
        ]
    }
}
