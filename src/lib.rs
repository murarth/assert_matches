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
/// 
/// assert_matches!(a, Foo::A(i) if i > 0 => assert!(i != 0));
/// ```
#[macro_export]
macro_rules! assert_matches {
    ( $e:expr , $pat:pat ) => {
        match $e {
            $pat => (),
            ref e => panic!("assertion failed: `{:?}` does not match `{}`",
                e, stringify!($pat))
        }
    };
    ( $e:expr , $pat:pat if $cond:expr ) => {
        match $e {
            $pat if $cond => (),
            ref e => panic!("assertion failed: `{:?}` does not match `{} if {}`",
                e, stringify!($pat), stringify!($cond))
        }
    };
    ( $e:expr , $pat:pat => $arm:expr ) => {
        match $e {
            $pat => $arm,
            ref e => panic!("assertion failed: `{:?}` does not match `{}`",
                e, stringify!($pat))
        }
    };
    ( $e:expr , $pat:pat if $cond:expr => $arm:expr ) => {
        match $e {
            $pat if $cond => $arm,
            ref e => panic!("assertion failed: `{:?}` does not match `{} if {}`",
                e, stringify!($pat), stringify!($cond))
        }
    };
    ( $e:expr , $pat:pat , $($arg:tt)* ) => {
        match $e {
            $pat => (),
            ref e => panic!("assertion failed: `{:?}` does not match `{}`: {}",
                e, stringify!($pat), format_args!($($arg)*))
        }
    };
    ( $e:expr , $pat:pat if $cond:expr , $($arg:tt)* ) => {
        match $e {
            $pat if $cond => (),
            ref e => panic!("assertion failed: `{:?}` does not match `{} if {}`: {}",
                e, stringify!($pat), stringify!($cond), format_args!($($arg)*))
        }
    };
    ( $e:expr , $pat:pat => $arm:expr , $($arg:tt)* ) => {
        match $e {
            $pat => $arm,
            ref e => panic!("assertion failed: `{:?}` does not match `{}`: {}",
                e, stringify!($pat), format_args!($($arg)*))
        }
    };
    ( $e:expr , $pat:pat if $cond:expr => $arm:expr , $($arg:tt)* ) => {
        match $e {
            $pat if $cond => $arm,
            ref e => panic!("assertion failed: `{:?}` does not match `{} if {}`: {}",
                e, stringify!($pat), stringify!($cond), format_args!($($arg)*))
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
        assert_matches!(b, Foo::B(s) => assert_eq!(s, "foo"));
        assert_matches!(b, Foo::B(s) => { assert_eq!(s, "foo"); assert!(true) });
        assert_matches!(b, Foo::B(s) if s == "foo" => assert_eq!(s, "foo"));
        assert_matches!(b, Foo::B(s) if s == "foo" => { assert_eq!(s, "foo"); assert!(true) });
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

    #[test]
    #[should_panic]
    fn test_assert_panic_3() {
        let b = Foo::B("foo");

        assert_matches!(b, Foo::B(s) => assert_eq!(s, "bar"));
    }

    #[test]
    #[should_panic]
    fn test_assert_panic_4() {
        let b = Foo::B("foo");

        assert_matches!(b, Foo::B(s) if s == "bar" => assert_eq!(s, "foo"));
    }

    #[test]
    #[should_panic]
    fn test_assert_panic_5() {
        let b = Foo::B("foo");

        assert_matches!(b, Foo::B(s) if s == "foo" => assert_eq!(s, "bar"));
    }

    #[test]
    #[should_panic]
    fn test_assert_panic_6() {
        let b = Foo::B("foo");

        assert_matches!(b, Foo::B(s) if s == "foo" => { assert_eq!(s, "foo"); assert!(false) });
    }

    #[test]
    fn test_assert_no_move() {
        let b = &mut Foo::A(0);
        assert_matches!(*b, Foo::A(0));
    }

    #[test]
    fn assert_with_message() {
        let a = Foo::A(0);

        assert_matches!(a, Foo::A(_), "o noes");
        assert_matches!(a, Foo::A(n) if n == 0, "o noes");
        assert_matches!(a, Foo::A(n) => assert_eq!(n, 0), "o noes");
        assert_matches!(a, Foo::A(n) => { assert_eq!(n, 0); assert!(n < 1) }, "o noes");
        assert_matches!(a, Foo::A(n) if n == 0 => assert_eq!(n, 0), "o noes");
        assert_matches!(a, Foo::A(n) if n == 0 => { assert_eq!(n, 0); assert!(n < 1) }, "o noes");
        assert_matches!(a, Foo::A(_), "o noes {:?}", a);
        assert_matches!(a, Foo::A(n) if n == 0, "o noes {:?}", a);
        assert_matches!(a, Foo::A(n) => assert_eq!(n, 0), "o noes {:?}", a);
        assert_matches!(a, Foo::A(n) => { assert_eq!(n, 0); assert!(n < 1) }, "o noes {:?}", a);
        assert_matches!(a, Foo::A(_), "o noes {value:?}", value=a);
        assert_matches!(a, Foo::A(n) if n == 0, "o noes {value:?}", value=a);
        assert_matches!(a, Foo::A(n) => assert_eq!(n, 0), "o noes {value:?}", value=a);
        assert_matches!(a, Foo::A(n) => { assert_eq!(n, 0); assert!(n < 1) }, "o noes {value:?}", value=a);
        assert_matches!(a, Foo::A(n) if n == 0 => assert_eq!(n, 0), "o noes {value:?}", value=a);
    }
}
