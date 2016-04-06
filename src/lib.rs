//! A simple set of utility traits for working with tuples

/// Helper trait to allow Appending of tuples
pub trait Append<T> {
    type Output;
    /// Append T onto the end of the tuple returning
    /// a new tuple with the existing elements and T
    fn append(self, T) -> Self::Output;
}

/// Helper trait to allow Perpending of tuples
pub trait Prepend<T> {
    type Output;
    /// Append T onto the start of the tuple returning
    /// a new tuple with all the elements from shifted
    /// over one row and T in the first slot
    fn prepend(self, T) -> Self::Output;
}

/// Helper trait that allow for merging of tuples
pub trait Merge<T> {
    type Output;
    /// merge LHS with RHS returning a new tuple
    /// that contains the elements of both tuples
    /// ordering is preserved such that LHS elements
    /// come before RHS elements.
    fn merge(self, T) -> Self::Output;
}

macro_rules! tuple_impl {
    // use variables to indicate the arity of the tuple
    ($($from:ident),*) => {
        // the trailing commas are for the 1 tuple
        impl<$($from),* , T> Append<T> for ( $( $from ),* ,) {
            type Output = ( $( $from ),*,  T);

            #[inline]
            #[allow(non_snake_case)]
            fn append(self, x: T) -> ( $( $from ),*,  T) {
                match self {
                    ($($from,)*) => ($($from,)* x)
                }
            }
        }

        // the trailing commas are for the 1 tuple
        impl<$($from),* , T> Prepend<T> for ( $( $from ),* ,) {
            type Output = (T, $( $from ),*);

            #[inline]
            #[allow(non_snake_case)]
            fn prepend(self, x: T) -> (T, $( $from ),*) {
                match self {
                    ($($from,)*) => (x, $($from,)*)
                }
            }
        }
    }
}

tuple_impl!{A}
tuple_impl!{A, B}
tuple_impl!{A, B, C}
tuple_impl!{A, B, C, D}
tuple_impl!{A, B, C, D, E}
tuple_impl!{A, B, C, D, E, F}
tuple_impl!{A, B, C, D, E, F, G}
tuple_impl!{A, B, C, D, E, F, G, H}
tuple_impl!{A, B, C, D, E, F, G, H, I}
tuple_impl!{A, B, C, D, E, F, G, H, I, J}
tuple_impl!{A, B, C, D, E, F, G, H, I, J, K}
tuple_impl!{A, B, C, D, E, F, G, H, I, J, K, L}
tuple_impl!{A, B, C, D, E, F, G, H, I, J, K, L, M}
tuple_impl!{A, B, C, D, E, F, G, H, I, J, K, L, M, N}
tuple_impl!{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O}
tuple_impl!{A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P}


macro_rules! merge_impl {
    ([$($a:ident),+] [$($b:ident),+]) => {
        // the trailing commas are for the 1 tuple
        impl<$($a,)* $($b,)*> Merge<($($b),+,)> for ( $($a),+ ,) {
            type Output = ($($a,)+ $($b,)+);

            #[inline]
            #[allow(non_snake_case)]
            fn merge(self, x: ($($b,)+)) -> ($($a,)+ $($b,)+) {
                match (self, x) {
                    (($($a,)+), ($($b,)+)) => ($($a,)+ $($b,)+)
                }
            }
        }
    };

    ([$($a:ident),+] []) => {
        // the trailing commas are for the 1 tuple
        impl<$($a,)*> Merge<()> for ( $($a),+ ,) {
            type Output = ($($a,)+);

            #[inline]
            #[allow(non_snake_case)]
            fn merge(self, _: ()) -> ($($a,)+) { self }
        }
    };

    ([] [$($a:ident),+]) => {
        // the trailing commas are for the 1 tuple
        impl<$($a,)*> Merge<( $($a),+ ,)> for () {
            type Output = ($($a),+ ,);

            #[inline]
            #[allow(non_snake_case)]
            fn merge(self, x: ($($a,)+),) -> ($($a),+,) { x }
        }
    };

    ([] []) => {
        // the trailing commas are for the 1 tuple
        impl Merge<()> for () {
            type Output = ();

            #[inline]
            #[allow(non_snake_case)]
            fn merge(self, _: ()) -> () { self }
        }
    };
}

// I wish I knew a better way....
// this is every possible LHS RHS type of a tuple that when merged
// would result in a tuple that was 16 elements or less.
merge_impl!{[] []}
merge_impl!{[] [A]}
merge_impl!{[] [A, B]}
merge_impl!{[] [A, B, C]}
merge_impl!{[] [A, B, C, D]}
merge_impl!{[] [A, B, C, D, E]}
merge_impl!{[] [A, B, C, D, E, F]}
merge_impl!{[] [A, B, C, D, E, F, G]}
merge_impl!{[] [A, B, C, D, E, F, G, H]}
merge_impl!{[] [A, B, C, D, E, F, G, H, I]}
merge_impl!{[] [A, B, C, D, E, F, G, H, I, J]}
merge_impl!{[] [A, B, C, D, E, F, G, H, I, J, K]}
merge_impl!{[] [A, B, C, D, E, F, G, H, I, J, K, L]}
merge_impl!{[] [A, B, C, D, E, F, G, H, I, J, K, L, M]}
merge_impl!{[] [A, B, C, D, E, F, G, H, I, J, K, L, M, N]}
merge_impl!{[] [A, B, C, D, E, F, G, H, I, J, K, L, M, N, O]}
merge_impl!{[] [A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P]}
merge_impl!{[A] []}
merge_impl!{[A] [B]}
merge_impl!{[A] [B, C]}
merge_impl!{[A] [B, C, D]}
merge_impl!{[A] [B, C, D, E]}
merge_impl!{[A] [B, C, D, E, F]}
merge_impl!{[A] [B, C, D, E, F, G]}
merge_impl!{[A] [B, C, D, E, F, G, H]}
merge_impl!{[A] [B, C, D, E, F, G, H, I]}
merge_impl!{[A] [B, C, D, E, F, G, H, I, J]}
merge_impl!{[A] [B, C, D, E, F, G, H, I, J, K]}
merge_impl!{[A] [B, C, D, E, F, G, H, I, J, K, L]}
merge_impl!{[A] [B, C, D, E, F, G, H, I, J, K, L, M]}
merge_impl!{[A] [B, C, D, E, F, G, H, I, J, K, L, M, N]}
merge_impl!{[A] [B, C, D, E, F, G, H, I, J, K, L, M, N, O]}
merge_impl!{[A] [B, C, D, E, F, G, H, I, J, K, L, M, N, O, P]}
merge_impl!{[A, B] []}
merge_impl!{[A, B] [C]}
merge_impl!{[A, B] [C, D]}
merge_impl!{[A, B] [C, D, E]}
merge_impl!{[A, B] [C, D, E, F]}
merge_impl!{[A, B] [C, D, E, F, G]}
merge_impl!{[A, B] [C, D, E, F, G, H]}
merge_impl!{[A, B] [C, D, E, F, G, H, I]}
merge_impl!{[A, B] [C, D, E, F, G, H, I, J]}
merge_impl!{[A, B] [C, D, E, F, G, H, I, J, K]}
merge_impl!{[A, B] [C, D, E, F, G, H, I, J, K, L]}
merge_impl!{[A, B] [C, D, E, F, G, H, I, J, K, L, M]}
merge_impl!{[A, B] [C, D, E, F, G, H, I, J, K, L, M, N]}
merge_impl!{[A, B] [C, D, E, F, G, H, I, J, K, L, M, N, O]}
merge_impl!{[A, B] [C, D, E, F, G, H, I, J, K, L, M, N, O, P]}
merge_impl!{[A, B, C] []}
merge_impl!{[A, B, C] [D]}
merge_impl!{[A, B, C] [D, E]}
merge_impl!{[A, B, C] [D, E, F]}
merge_impl!{[A, B, C] [D, E, F, G]}
merge_impl!{[A, B, C] [D, E, F, G, H]}
merge_impl!{[A, B, C] [D, E, F, G, H, I]}
merge_impl!{[A, B, C] [D, E, F, G, H, I, J]}
merge_impl!{[A, B, C] [D, E, F, G, H, I, J, K]}
merge_impl!{[A, B, C] [D, E, F, G, H, I, J, K, L]}
merge_impl!{[A, B, C] [D, E, F, G, H, I, J, K, L, M]}
merge_impl!{[A, B, C] [D, E, F, G, H, I, J, K, L, M, N]}
merge_impl!{[A, B, C] [D, E, F, G, H, I, J, K, L, M, N, O]}
merge_impl!{[A, B, C] [D, E, F, G, H, I, J, K, L, M, N, O, P]}
merge_impl!{[A, B, C, D] []}
merge_impl!{[A, B, C, D] [E]}
merge_impl!{[A, B, C, D] [E, F]}
merge_impl!{[A, B, C, D] [E, F, G]}
merge_impl!{[A, B, C, D] [E, F, G, H]}
merge_impl!{[A, B, C, D] [E, F, G, H, I]}
merge_impl!{[A, B, C, D] [E, F, G, H, I, J]}
merge_impl!{[A, B, C, D] [E, F, G, H, I, J, K]}
merge_impl!{[A, B, C, D] [E, F, G, H, I, J, K, L]}
merge_impl!{[A, B, C, D] [E, F, G, H, I, J, K, L, M]}
merge_impl!{[A, B, C, D] [E, F, G, H, I, J, K, L, M, N]}
merge_impl!{[A, B, C, D] [E, F, G, H, I, J, K, L, M, N, O]}
merge_impl!{[A, B, C, D] [E, F, G, H, I, J, K, L, M, N, O, P]}
merge_impl!{[A, B, C, D, E] []}
merge_impl!{[A, B, C, D, E] [F]}
merge_impl!{[A, B, C, D, E] [F, G]}
merge_impl!{[A, B, C, D, E] [F, G, H]}
merge_impl!{[A, B, C, D, E] [F, G, H, I]}
merge_impl!{[A, B, C, D, E] [F, G, H, I, J]}
merge_impl!{[A, B, C, D, E] [F, G, H, I, J, K]}
merge_impl!{[A, B, C, D, E] [F, G, H, I, J, K, L]}
merge_impl!{[A, B, C, D, E] [F, G, H, I, J, K, L, M]}
merge_impl!{[A, B, C, D, E] [F, G, H, I, J, K, L, M, N]}
merge_impl!{[A, B, C, D, E] [F, G, H, I, J, K, L, M, N, O]}
merge_impl!{[A, B, C, D, E] [F, G, H, I, J, K, L, M, N, O, P]}
merge_impl!{[A, B, C, D, E, F] []}
merge_impl!{[A, B, C, D, E, F] [G]}
merge_impl!{[A, B, C, D, E, F] [G, H]}
merge_impl!{[A, B, C, D, E, F] [G, H, I]}
merge_impl!{[A, B, C, D, E, F] [G, H, I, J]}
merge_impl!{[A, B, C, D, E, F] [G, H, I, J, K]}
merge_impl!{[A, B, C, D, E, F] [G, H, I, J, K, L]}
merge_impl!{[A, B, C, D, E, F] [G, H, I, J, K, L, M]}
merge_impl!{[A, B, C, D, E, F] [G, H, I, J, K, L, M, N]}
merge_impl!{[A, B, C, D, E, F] [G, H, I, J, K, L, M, N, O]}
merge_impl!{[A, B, C, D, E, F] [G, H, I, J, K, L, M, N, O, P]}
merge_impl!{[A, B, C, D, E, F, G] []}
merge_impl!{[A, B, C, D, E, F, G] [H]}
merge_impl!{[A, B, C, D, E, F, G] [H, I]}
merge_impl!{[A, B, C, D, E, F, G] [H, I, J]}
merge_impl!{[A, B, C, D, E, F, G] [H, I, J, K]}
merge_impl!{[A, B, C, D, E, F, G] [H, I, J, K, L]}
merge_impl!{[A, B, C, D, E, F, G] [H, I, J, K, L, M]}
merge_impl!{[A, B, C, D, E, F, G] [H, I, J, K, L, M, N]}
merge_impl!{[A, B, C, D, E, F, G] [H, I, J, K, L, M, N, O]}
merge_impl!{[A, B, C, D, E, F, G] [H, I, J, K, L, M, N, O, P]}
merge_impl!{[A, B, C, D, E, F, G, H] []}
merge_impl!{[A, B, C, D, E, F, G, H] [I]}
merge_impl!{[A, B, C, D, E, F, G, H] [I, J]}
merge_impl!{[A, B, C, D, E, F, G, H] [I, J, K]}
merge_impl!{[A, B, C, D, E, F, G, H] [I, J, K, L]}
merge_impl!{[A, B, C, D, E, F, G, H] [I, J, K, L, M]}
merge_impl!{[A, B, C, D, E, F, G, H] [I, J, K, L, M, N]}
merge_impl!{[A, B, C, D, E, F, G, H] [I, J, K, L, M, N, O]}
merge_impl!{[A, B, C, D, E, F, G, H] [I, J, K, L, M, N, O, P]}
merge_impl!{[A, B, C, D, E, F, G, H, I] []}
merge_impl!{[A, B, C, D, E, F, G, H, I] [J]}
merge_impl!{[A, B, C, D, E, F, G, H, I] [J, K]}
merge_impl!{[A, B, C, D, E, F, G, H, I] [J, K, L]}
merge_impl!{[A, B, C, D, E, F, G, H, I] [J, K, L, M]}
merge_impl!{[A, B, C, D, E, F, G, H, I] [J, K, L, M, N]}
merge_impl!{[A, B, C, D, E, F, G, H, I] [J, K, L, M, N, O]}
merge_impl!{[A, B, C, D, E, F, G, H, I] [J, K, L, M, N, O, P]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J] []}
merge_impl!{[A, B, C, D, E, F, G, H, I, J] [K]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J] [K, L]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J] [K, L, M]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J] [K, L, M, N]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J] [K, L, M, N, O]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J] [K, L, M, N, O, P]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K] []}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K] [L]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K] [L, M]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K] [L, M, N]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K] [L, M, N, O]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K] [L, M, N, O, P]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L] []}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L] [M]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L] [M, N]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L] [M, N, O]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L] [M, N, O, P]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L, M] []}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L, M] [N]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L, M] [N, O]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L, M] [N, O, P]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L, M, N] []}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L, M, N] [O]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L, M, N] [O, P]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L, M, N, O] []}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L, M, N, O] [P]}
merge_impl!{[A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P] []}



#[cfg(test)]
mod test {
    use {Append, Prepend, Merge};

    #[test]
    fn append() {
        let out = (0,).append(1);
        assert_eq!(out.0, 0);
        assert_eq!(out.1, 1);
        let out = out.append(2);
        assert_eq!(out.0, 0);
        assert_eq!(out.1, 1);
        assert_eq!(out.2, 2);
        let out = out.append(3);
        assert_eq!(out.0, 0);
        assert_eq!(out.1, 1);
        assert_eq!(out.2, 2);
        assert_eq!(out.3, 3);
        let out = out.append("foo");
        assert_eq!(out.0, 0);
        assert_eq!(out.1, 1);
        assert_eq!(out.2, 2);
        assert_eq!(out.3, 3);
        assert_eq!(out.4, "foo");
    }

    #[test]
    fn prepend() {
        let out = (0,).prepend(1);
        assert_eq!(out.0, 1);
        assert_eq!(out.1, 0);
        let out = out.prepend(2);
        assert_eq!(out.0, 2);
        assert_eq!(out.1, 1);
        assert_eq!(out.2, 0);
        let out = out.prepend(3);
        assert_eq!(out.0, 3);
        assert_eq!(out.1, 2);
        assert_eq!(out.2, 1);
        assert_eq!(out.3, 0);
        let out = out.prepend("foo");
        assert_eq!(out.0, "foo");
        assert_eq!(out.1, 3);
        assert_eq!(out.2, 2);
        assert_eq!(out.3, 1);
        assert_eq!(out.4, 0);
    }

    #[test]
    fn merge() {
        let a = (1, 2, 3);
        let b = ("foo", "bar");
        let c = a.merge(b);
        assert_eq!(c.0, 1);
        assert_eq!(c.1, 2);
        assert_eq!(c.2, 3);
        assert_eq!(c.3, "foo");
        assert_eq!(c.4, "bar");

        // assert to see if adding an empty
        // tuple results in a tuple.
        let a = ("test",);
        let out = a.merge(());
        assert_eq!(out.0, "test");

        let a = ("test",);
        let out = ().merge(a);
        assert_eq!(out.0, "test");
    }
}