//! Provides a macro, `assert_matches`, which tests whether a value
//! matches a given pattern, causing a panic if the match fails.

#![deny(missing_docs)]

/// Asserts that an expression matches a given pattern, with an optional guard
/// expression.
///
/// ```ignore
/// #[macro_use] extern crate assert_matches;
///
/// #[derive(Debug)]
/// enum Foo {
///     A(i32),
///     B(i32),
/// }
///
/// let a = Foo::A(1);
///
/// assert_matches!(a, Foo::A(_));
///
/// assert_matches!(a, Foo::A(i) if i > 0);
/// ```
#[macro_export]
macro_rules! assert_matches {
    ( $e:expr , $pat:pat ) => {
        match $e {
            $pat => (),
            e => panic!("assertion failed: `{:?}` does not match `{}`",
                e, stringify!($pat))
        }
    };
    ( $e:expr , $pat:pat if $cond:expr ) => {
        match $e {
            $pat if $cond => (),
            e => panic!("assertion failed: `{:?}` does not match `{} if {}`",
                e, stringify!($pat), stringify!($cond))
        }
    };
}

#[cfg(test)]
mod test {
    #[derive(Debug)]
    enum Foo {
        A(i32),
        B(&'static str),
    }

    #[test]
    fn test_assert_succeed() {
        let a = Foo::A(123);

        assert_matches!(a, Foo::A(_));
        assert_matches!(a, Foo::A(123));
        assert_matches!(a, Foo::A(i) if i == 123);

        let b = Foo::B("foo");

        assert_matches!(b, Foo::B(_));
        assert_matches!(b, Foo::B("foo"));
        assert_matches!(b, Foo::B(s) if s == "foo");
    }

    #[test]
    #[should_panic]
    fn test_assert_panic_0() {
        let a = Foo::A(123);

        assert_matches!(a, Foo::B(_));
    }

    #[test]
    #[should_panic]
    fn test_assert_panic_1() {
        let b = Foo::B("foo");

        assert_matches!(b, Foo::B("bar"));
    }

    #[test]
    #[should_panic]
    fn test_assert_panic_2() {
        let b = Foo::B("foo");

        assert_matches!(b, Foo::B(s) if s == "bar");
    }
}
