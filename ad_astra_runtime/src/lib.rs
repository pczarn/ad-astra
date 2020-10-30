extern crate proc_macro;
extern crate cfg;
extern crate gearley;
extern crate internship;

mod interner;
#[macro_use]
mod macros;

use std::collections::BTreeMap;

use cfg::earley::Grammar;
use cfg::Symbol;
use gearley::grammar::InternalGrammar;
use gearley::recognizer::Recognizer;
use gearley::forest::NullForest;

use self::interner::Interner;

pub struct NeighborhoodRuntime<T> {
    stmts: Vec<Neighborhood<T>>,
    param_map: Interner,
    external_syms: BTreeMap<ExtPath, Symbol>,
    external_grammar: Grammar,
    internal_grammar: Option<InternalGrammar>,
}

impl<T> NeighborhoodRuntime<T> {
    pub fn new() -> Self {
        NeighborhoodRuntime {
            stmts: Vec::new(),
            param_map: Interner::new(),
            external_syms: BTreeMap::new(),
            external_grammar: Grammar::new(),
            internal_grammar: None,
        }
    }

    pub fn rule(&mut self, neighborhood: Neighborhood<T>) {
        self.stmts.push(neighborhood);
    }

    pub fn process_rules(&mut self) {
        let mut neighborhoods = neighborhood.separate_alternatives();
        for neighborhood in &mut neighborhoods {
            neighborhood.externalize();
        }
        let (start, nested, terminal) = self.external_grammar.sym();
        external.rule(start).rhs([nested, terminal])
                .rule(nested).rhs([terminal, terminal]);
        external.set_start(start);
        let cfg = InternalGrammar::from_grammar(&external);
    }

    pub fn validate_steps(&self, steps: &[T]) {
        let mut rec = Recognizer::new(&cfg, NullForest);
        for step in steps {
            recognizer.scan();
        }
    }
}

impl<T> Path<T> {
    fn segments(&self) -> Vec<Vec<Step>> {
        self.steps
    }
}

// #[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
// pub enum Lhs {
//     Symbol(String),
//     Param {
//         rhs: String,
//         ty_param: String,
//     },
// }

pub enum Matcher<T> {
    Symbol(String),
    ParamApply {
        rhs: String,
        ty_param: String,
    },
    Pattern(Box<dyn Fn(&T) -> bool>),

    // Processed.
    ExtSymbol(Symbol),
    ExtParamApply {
        rhs: Symbol,
        ty_param: usize,
    },
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Step {
    Alternative(usize),
    Idx(usize),
    Repeat,

    // Lhs can come after param introduction.
    IntroduceParam(String),
    Lhs,
    Rhs,

    // Processed.
    ExtIntroduceParam(usize),
    ExtAlternativeAndIdx(Vec<usize>, usize),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ExtStep {
    Repeat,

    // Lhs can come after param introduction.
    Lhs,
    Rhs,

    // Processed.
    AlternativeAndIdx(Vec<usize>, usize),
}

pub struct Path<T> {
    steps: Vec<Step>,
    matcher: Matcher<T>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct ExtPath {
    steps: Vec<ExtStep>,
    symbol: Symbol,
}

pub struct Neighborhood<T> {
    paths: Vec<Path<T>>,
}

impl<T> Neighborhood<T> {
    fn new() -> Self {
        Neighborhood {
            paths: vec![]
        }
    }
}

// impl Lhs {
//     fn variant(s: &str) -> Lhs {
//         Lhs::Symbol(s.to_string())
//     }

//     fn apply(ty: &str, parameter: &str) -> Lhs {
//         Lhs::Param { rhs: ty.to_string(), ty_param: parameter.to_string() }
//     }
// }

// impl<T> Into<Lhs> for Matcher<T> {
//     fn into(self) -> Lhs {
//         match self {
//             Matcher::Symbol(sym) => {
//                 Lhs::Symbol(sym)
//             }
//             Matcher::Param { rhs, ty_param } => {
//                 Lhs::Param { rhs, ty_param }
//             }
//             Matcher::Pattern(_) => panic!("invalid conversion")
//         }
//     }
// }

impl Step {
    fn is_alternative(&self) -> bool {
        match self {
            &Step::Alternative(_) => true,
            _ => false,
        }
    }

    fn is_idx(&self) -> bool {
        match self {
            &Step::Idx(_) => true,
            _ => false,
        }
    }
}

impl<U> Matcher<U> {
    pub fn variant(s: &str) -> Matcher<U> {
        Matcher::Symbol(s.to_string())
    }

    pub fn apply(ty: &str, parameter: &str) -> Matcher<U> {
        Matcher::Param { rhs: ty.to_string(), ty_param: parameter.to_string() }
    }

    pub fn match_pattern(func: Box<dyn Fn(&U) -> bool>) -> Matcher<U> {
        Matcher::Pattern(func)
    }

    pub fn into_neighborhood(self) -> Neighborhood<U> {
        Neighborhood {
            paths: vec![
                Path {
                    steps: vec![],
                    matcher: self,
                }
            ]
        }
    }
}

impl<T> Neighborhood<T> {
    pub fn introduce_param(mut self, param: &str) -> Neighborhood<T> {
        for path in &mut self.paths {
            path.steps.insert(0, Step::IntroduceParam(param.to_string()));
        }
        self
    }

    pub fn repeat(mut self) -> Neighborhood<T> {
        for path in &mut self.paths {
            path.steps.insert(0, Step::Repeat);
        }
        self
    }

    pub fn then(mut self, mut next: Neighborhood<T>) -> Neighborhood<T> {
        if !self.paths.iter().all(|path| path.steps.get(0).map_or(false, |step| step.is_idx())) {
            for path in &mut self.paths {
                path.steps.insert(0, Step::Idx(0));
            }
        }
        let last_num = self.paths.iter().map(|path|
            match path.steps[0] {
                Step::Idx(n) => n,
                _ => unreachable!()
            }
        ).max().map_or(-1, |n| n as isize);
        for path in &mut next.paths {
            path.steps.insert(0, Step::Idx((last_num + 1) as usize));
        }
        self.paths.extend(next.paths.into_iter());
        self
    }

    pub fn or(mut self, mut next: Neighborhood<T>) -> Neighborhood<T> {
        if !self.paths.iter().all(|path| path.steps.get(0).map_or(false, |step| step.is_idx())) {
            for path in &mut self.paths {
                path.steps.insert(0, Step::Alternative(0));
            }
        }
        let last_num = self.paths.iter().map(|path|
            match path.steps[0] {
                Step::Alternative(n) => n,
                _ => unreachable!()
            }
        ).max().map_or(-1, |n| n as isize);
        for path in &mut next.paths {
            path.steps.insert(0, Step::Alternative((last_num + 1) as usize));
        }
        self.paths.extend(next.paths.into_iter());
        self
    }

    pub fn offshoot(mut self, mut next: Neighborhood<T>) -> Neighborhood<T> {
        if !self.paths.iter().all(|path| path.steps.get(0).map_or(false, |step| step.is_idx())) {
            for path in &mut self.paths {
                path.steps.insert(0, Step::Alternative(0));
            }
        }
        let last_num = self.paths.iter().map(|path|
            match path.steps[0] {
                Step::Alternative(n) => n,
                _ => unreachable!()
            }
        ).max().map_or(-1, |n| n as isize);
        for path in &mut next.paths {
            path.steps.insert(0, Step::Alternative((last_num + 1) as usize));
        }
        self.paths.extend(next.paths.into_iter());
        self
    }

    pub fn lhs_then(mut self, mut rhs: Neighborhood<T>) -> Self {
        assert_eq!(self.paths.len(), 1);
        for lhs_path in &mut self.paths {
            lhs_path.steps.insert(0, Step::Lhs);
        }
        for mut rhs_path in rhs.paths {
            rhs_path.steps.insert(0, Step::Rhs);
            self.paths.push(rhs_path);
        }
        rhs
    }

    fn separate_alternatives(self) -> Vec<Neighborhood<T>> {

    }

    fn externalize(&mut self) {
        let mut new_paths = vec![];
        for path in &mut self.paths {
            for i in 0 .. path.steps.len() {
                match path.steps[i] {
                    Step::Rhs => {
                        let mut used_indices = vec![];
                        let mut alts = vec![];
                        let mut idx = None;
                        for j in i + 1 .. path.steps.len() {
                            match path.steps[j] {
                                Step::Alternative(i) => {
                                    alts.push(i);
                                    used_indices.push(j);
                                }
                                Step::Idx(i) => {
                                    idx = Some(i);
                                    used_indices.push(j);
                                    break;
                                }
                                Step::Repeat | Step::IntroduceParam(..) => break,
                                Step::Rhs | Step::AlternativeAndIdx(..) => unreachable!()
                            }
                        }
                        used_indices.sort();
                        for i in used_indices.into_iter().rev() {
                            path.steps.remove(i);
                        }
                        path.steps[i] = Step::AlternativeAndIdx(alts, idx.unwrap_or(0));
                    }
                    Step::Alternative(i) => {
                        let mut used_indices = vec![];
                        let mut alts = vec![i];
                        let mut idx = None;
                        for j in i + 1 .. path.steps.len() {
                            match path.steps[j] {
                                Step::Alternative(i) => {
                                    alts.push(i);
                                    used_indices.push(j);
                                }
                                Step::Idx(i) => {
                                    idx = Some(i);
                                    used_indices.push(j);
                                    break;
                                }
                                Step::Repeat | Step::IntroduceParam(..) => break,
                                Step::Rhs | Step::AlternativeAndIdx(..) => unreachable!()
                            }
                        }
                        used_indices.sort();
                        for i in used_indices.into_iter().rev() {
                            path.steps.remove(i);
                        }
                        path.steps[i] = Step::AlternativeAndIdx(alts, idx.unwrap_or(0));
                    }
                    Step::Idx(i) => {
                        path.steps[i] = Step::AlternativeAndIdx(vec![], i);
                    }
                    _ => {}
                }
            }
        }

        for i in 0 .. self.paths.len() {
            let mut used_indices = vec![];
            for j in 0 .. self.paths[i].steps.len() {
                match self.paths[i].steps[j] {
                    Step::IntroduceParam(ref param_name) => {
                        let mut k = i + 1;
                        while k < self.paths.len() &&
                                self.paths[k].steps[.. j + 1].iter().enumerate().all(|(l, step)| step == self.paths[i].steps[l]) {
                                self.paths[k].steps[j] = Step::ExtIntroduceParam()
                                continue;
                            }
                            break;
                        }
                        used_indices.push(j);
                    }
                    _ => {}
                }
            }
            used_indices.sort();
            for j in used_indices.into_iter().rev() {
                path.steps.remove(j);
            }
        }

        for path in &mut self.paths {
            for i in 1 .. path.steps.len() {
                let path = Path { steps: path.steps[.. i].to_vec() };
                self.external_syms.entry(path).or_insert_with(|| self.external_grammar.sym());
            }
        }
    }
}
