//! A simple set of utility traits for working with tuples

/// Helper trait to allow Appending of tuples
pub trait Append<T> {
    type Output : PluckTail<Head = Self, Tail = T>;
    /// Append T onto the end of the tuple returning
    /// a new tuple with the existing elements and T
    fn append(self, other: T) -> Self::Output;
}

/// Helper trait to allow Plucking **tails** of tuples.
///
/// This is the inverse of [`Append`]
pub trait PluckTail {
    type Head : Append<Self::Tail, Output = Self>;
    type Tail;
    /// Split the tuple into the tail (`Tail`) and the rest part (`Head`)
    fn pluck_tail(self) -> (Self::Head, Self::Tail);
}

/// Helper trait to allow Perpending of tuples
pub trait Prepend<T> {
    type Output : Pluck<Head = T, Tail = Self>;
    /// Append T onto the start of the tuple returning
    /// a new tuple with all the elements from shifted
    /// over one row and T in the first slot
    fn prepend(self, other: T) -> Self::Output;
}

/// Helper trait to allow Plucking **heads** of tuples.
///
/// This is the inverse of [`Prepend`]
pub trait Pluck {
    type Head;
    type Tail : Prepend<Self::Head, Output = Self>;
    /// Split the tuple into the head (`Head`) and the rest part (`Tail`)
    fn pluck(self) -> (Self::Head, Self::Tail);
}

pub trait Call<T, O> {
    fn call(f : Self, t : T) -> O;
}

pub trait RefCall<T, O> {
    fn ref_call(f : Self, t : &T) -> O;
}

macro_rules! tuple_impl {
    // use variables to indicate the arity of the tuple
    ($($from:ident,)*) => {
        // the trailing commas are for the 1 tuple
        impl<$($from,)* T> Append<T> for ( $( $from ,)* ) {
            type Output = ( $( $from ,)*  T, );

            #[inline]
            #[allow(non_snake_case)]
            fn append(self, x: T) -> ( $( $from ,)*  T,) {
                match self {
                    ($($from,)*) => ($($from,)* x,)
                }
            }
        }

        impl<$($from,)* T> PluckTail for ($($from,)* T,) {
            type Head = ($($from,)*);
            type Tail = T;

            #[inline]
            #[allow(non_snake_case)]
            fn pluck_tail(self) -> (Self::Head, Self::Tail) {
                match self {
                    ($($from,)* x,) => (($($from,)*), x)
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

        impl<$($from,)* T> Pluck for (T, $($from,)*) {
            type Head = T;
            type Tail = ($($from,)*);

            #[inline]
            #[allow(non_snake_case)]
            fn pluck(self) -> (Self::Head, Self::Tail) {
                match self {
                    (x, $($from,)*) => (x, ($($from,)*))
                }
            }
        }

        impl<$($from,)* F, O> RefCall<($($from,)*), O> for F
        where
            F : for<'a> FnOnce($(&'a $from,)*) -> O
        {
            #[inline]
            #[allow(non_snake_case)]
            fn ref_call(f : F, _t : &($($from,)*)) -> O {

                let ($(ref $from,)*) = _t;

                f($($from,)*)
            }
        }

        impl<$($from,)* F, O> Call<($($from,)*), O> for F
        where
            F : FnOnce($($from,)*) -> O
        {
            #[inline]
            #[allow(non_snake_case)]
            fn call(f : F, _t : ($($from,)*)) -> O {

                let ($($from,)*) = _t;

                f($($from,)*)
            }
        }
    }
}

macro_rules! for_each_prefix (
    ($m:ident, [$(($arg:tt),)*]) => {
        for_each_prefix!($m, [], [$(($arg),)*]);
    };
    ($m:ident, [$(($acc:tt),)*], []) => {
        $m!($($acc,)*);
    };
    ($m:ident, [$(($acc:tt),)*], [($arg0:tt), $(($arg:tt),)*]) => {
        $m!($($acc,)*);
        for_each_prefix!($m, [$(($acc),)* ($arg0),], [$(($arg),)*]);
    };
);

for_each_prefix! {
    tuple_impl,
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
    fn merge(self, other: T) -> Self::Output;
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

for_each_prefix! {
    merge_impl2,
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

for_each_prefix! {
    split_impl,
    [((T0, T1)), ((T2, T3)), ((T4, T5)), ((T6, T7)), ((T8, T9)), ((T10, T11)), ((T12, T13)), ((T14, T15)),]
}

#[cfg(test)]
mod test {
    use {Append, Merge, Pluck, PluckTail, Prepend, Split, Call, RefCall};

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

        let (head, tail) = out.pluck_tail();
        assert_eq!((head, tail), ((0, 1, 2, 3), "foo"));
        let (head, tail) = head.pluck_tail();
        assert_eq!((head, tail), ((0, 1, 2), 3));
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

        let (head, tail) = out.pluck();
        assert_eq!((head, tail), ("foo", (3, 2, 1, 0)));
        let (head, tail) = tail.pluck();
        assert_eq!((head, tail), (3, (2, 1, 0)));
    }

    #[test]
    fn call() {
        let args = (1, "abc", true);
        let f = |a : usize, b : &str, c : bool| -> usize {
            if c {
                a
            } else {
                b.len()
            }
        };

        Call::call(f, args.clone());
        Call::call(f, args);

        let args = (1, "abc", true);
        let f = |a : &usize, b : &&str, c : &bool| -> usize {
            if *c {
                *a
            } else {
                b.len()
            }
        };

        RefCall::ref_call(f, &args);
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
        assert_eq!((1, 2).split(), ((1,), (2,)));
        assert_eq!((1, 2, 3).split(), ((1,), (2, 3)));
    }
}
