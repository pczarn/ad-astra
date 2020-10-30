#[macro_export]
macro_rules! ast {
    (
        ($d:tt), $Path:ident
    ) => {
        macro_rules! path {
            (
                $d ( $d e:expr),*
            ) => (
                $Path::with_steps(vec![$d ($d e),*])
            );
        }
    };
    (
        $Neighborhood:ident, $Path:ident, $Step:ident, ($($start_lhs:tt)*) =>
            $(
                (
                    $($rule:tt)*
                    // $lhs:tt
                    // ::= 
                    // $(
                    //     $rhs:tt
                    // )|+
                );
            )*
    ) => (
        ast!(($), $Path);

        struct $Neighborhood {
            paths: Vec<$Path>,
            runtime: $crate::NeighborhoodRuntime<$Step>,
        }

        struct $Path {
            steps: Vec<$Step>,
        }

        impl $Neighborhood {
            pub fn new() -> Self {
                let mut runtime = $crate::NeighborhoodRuntime::<$Step>::new();

                $(
                    runtime.rule(
                        // rule!($lhs).lhs_then(rule!($($rhs)|+))
                        rule!(( $($rule)* ));
                    );
                )*

                runtime.process_rules();

                $Neighborhood {
                    paths: Vec::new(),
                    runtime,
                }
            }

            pub fn with_paths(paths: Vec<$Path>) -> Self {
                let mut this = $Neighborhood::new();
                this.paths = paths;
                this
            }

            pub fn validate(&self) -> bool {
                let mut valid = true;
                for path in &self.paths {
                    valid = valid && self.runtime.validate_steps(&path.steps[..]);
                }
                valid
            }
        }

        impl $Path {
            pub fn new() -> Self {
                $Path { steps: Vec::new() }
            }

            pub fn with_steps(steps: Vec<$Step>) -> Self {
                $Path { steps }
            }
        }
    )
}

// #[macro_export]
// macro_rules! lhs {
//     ($lhs:ident) => {
//         $crate::Lhs::variant(stringify!($lhs))
//     };
//     (( $lhs:ident<$T:ty> )) => {
//         $crate::Lhs::apply(stringify!($lhs), stringify!($T))
//     };
//     (( for<$T:ident> $inner:tt )) => {
//         if let Lhs::Param { rhs, ty_param } = lhs!($inner) {
//             assert_eq!(ty_param, stringify!($T));
//             $crate::Lhs::Param { rhs, ty_param }
//         } else {
//             panic!("invalid LHS");
//         }
//     };
// }

#[macro_export]
macro_rules! rule {
    ($rhs:ident) => {
        $crate::Matcher::variant(stringify!($rhs)).into_neighborhood()
    };
    (( $rhs:ident<$T:ty> )) => {
        $crate::Matcher::apply(stringify!($rhs), stringify!($T)).into_neighborhood()
    };
    ((@m $pattern:pat )) => {
        $crate::Matcher::match_pattern(Box::new(|val| match val { &$pattern => true, _ => false }))
          .into_neighborhood()
    };
    (( for<$T:ident> $rhs:tt )) => {
        rule!($rhs).introduce_param(stringify!($T))
    };
    (( $rhs:tt * )) => {
        rule!($rhs).repeat()
    };
    (( $lhs:tt ::= $rhs:tt )) => {
        rule!($lhs).lhs_then(rule!($rhs))
    };
    ((
        /**/ $rhs0:tt
        $(^  $rhsN:tt)+
    )) => {
        /***/ rule!($rhs0)
        $(.offshoot(rule!($rhsN)))+
    };
    (( $rhs0:tt $($rhsN:tt)* )) => {
        rule!($rhs0) $(.then(rule!($rhsN)))*
    };
    (
        /**/ $rhs0:tt
        $(|  $rhsN:tt)+
    ) => {
        /***/ rule!($rhs0)
        $(.or(rule!($rhsN)))+
    };
}
