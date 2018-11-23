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

macro_rules! tuple_impl {
    // use variables to indicate the arity of the tuple
    ($($from:ident,)*) => {
        // the trailing commas are for the 1 tuple
        impl<$($from,)* T> Append<T> for ( $( $from ,)* ) {
            type Output = ( $( $from ,)*  T);

            #[inline]
            #[allow(non_snake_case)]
            fn append(self, x: T) -> ( $( $from ,)*  T) {
                match self {
                    ($($from,)*) => ($($from,)* x)
                }
            }
        }

        // the trailing commas are for the 1 tuple
        impl<$($from,)*  T> Prepend<T> for ( $( $from ,)* ) {
            type Output = (T, $( $from ,)*);

            #[inline]
            #[allow(non_snake_case)]
            fn prepend(self, x: T) -> (T, $( $from ,)*) {
                match self {
                    ($($from,)*) => (x, $($from,)*)
                }
            }
        }
    }
}

macro_rules! for_each_prefix (
    ($m:ident, [$(($acc:tt),)*], []) => {
        $m!($($acc,)*);
    };
    ($m:ident, [$(($acc:tt),)*], [($arg0:tt), $(($arg:tt),)*]) => {
        $m!($($acc,)*);
        for_each_prefix!($m, [$(($acc),)* ($arg0),], [$(($arg),)*]);
    };
);

for_each_prefix!{
    tuple_impl,
    [],
    [(T0), (T1), (T2), (T3), (T4), (T5), (T6), (T7), (T8), (T9), (T10), (T11), (T12), (T13), (T14), (T15),]
}

macro_rules! merge_impl {
    ([$($a:ident,)*], [$($b:ident,)*]) => {
        // the trailing commas are for the 1 tuple
        impl<$($a,)* $($b,)*> Merge<($($b,)*)> for ( $($a,)* ) {
            type Output = ($($a,)* $($b,)*);

            #[inline]
            #[allow(non_snake_case)]
            fn merge(self, x: ($($b,)*)) -> ($($a,)* $($b,)*) {
                match (self, x) {
                    (($($a,)*), ($($b,)*)) => ($($a,)* $($b,)*)
                }
            }
        }
    };
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

macro_rules! for_each_prefix_suffix (
    ($m:ident, [$(($acc:tt),)*], []) => {
        $m!([$($acc,)*], []);
    };
    ($m:ident, [$(($acc:tt),)*], [($arg0:tt), $(($arg:tt),)*]) => {
        $m!([$($acc,)*], [$arg0, $($arg,)*]);
        for_each_prefix_suffix!($m, [$(($acc),)* ($arg0),], [$(($arg),)*]);
    };
);

macro_rules! merge_impl2(
    ($($a: ident,)*) => (
        for_each_prefix_suffix!(
            merge_impl,
            [],
            [$(($a),)*]
        );
    );
);

for_each_prefix!{
    merge_impl2,
    [],
    [(T0), (T1), (T2), (T3), (T4), (T5), (T6), (T7), (T8), (T9), (T10), (T11), (T12), (T13), (T14), (T15),]
}

/// Tries to split a tuple into two tuples
/// if the tuple is odd sized the Right side will
/// contain the extra element
pub trait Split {
    type Left;
    type Right;

    fn split(self) -> (Self::Left, Self::Right);
}

macro_rules! split_impl (
    ($(($a:ident, $b:ident),)*) => (
        impl<$($a,)* $($b,)*> Split for ($($a,)* $($b,)*) {
            type Left = ($($a,)*);
            type Right = ($($b,)*);
            #[allow(non_snake_case)]
            fn split(self) -> (Self::Left, Self::Right) {
                match self {
                    ($($a,)* $($b,)*) => (($($a,)*), ($($b,)*))
                }
            }
        }
        impl<$($a,)* $($b,)* TLast> Split for ($($a,)* $($b,)* TLast,) {
            type Left = ($($a,)*);
            type Right = ($($b,)* TLast,);
            #[allow(non_snake_case)]
            fn split(self) -> (Self::Left, Self::Right) {
                match self {
                    ($($a,)* $($b,)* t_last,) => (($($a,)*), ($($b,)* t_last,))
                }
            }
        }
    );
);

for_each_prefix!{
    split_impl,
    [],
    [((T0, T1)), ((T2, T3)), ((T4, T5)), ((T6, T7)), ((T8, T9)), ((T10, T11)), ((T12, T13)), ((T14, T15)),]
}

#[cfg(test)]
mod test {
    use {Append, Prepend, Merge, Split};

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
        assert_eq!(().merge(()), ());
        assert_eq!(().merge((1,)), (1,));
        assert_eq!((1,).merge(()), (1,));
    }

    #[test]
    fn split() {
        let a = (1, 2, 3);
        let (b, c) = a.split();
        assert_eq!(b.0, 1);
        assert_eq!(c.0, 2);
        assert_eq!(c.1, 3);
        assert_eq!(().split(), ((), ()));
        assert_eq!((1,).split(), ((), (1,)));
        assert_eq!((1,2).split(), ((1,), (2,)));
        assert_eq!((1,2,3).split(), ((1,), (2,3)));
    }
}
