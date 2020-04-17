#![feature(proc_macro_hygiene)]

use irrefutable::irrefutable;
use std::ops::Range;

#[cfg(test)]
mod panic {
    use super::*;

    #[test]
    #[should_panic(expected = "The panic cause.")]
    fn tuple_struct() {
        #[irrefutable(panic("The panic cause."))]
        let Some("a") = Some("b");
    }
}

#[cfg(test)]
mod r#return {
    use super::*;

    #[test]
    fn tuple_struct() {
        #[irrefutable(return)]
        let Some("a") = Some("b");
        panic!("unreachable");
    }
}

#[cfg(test)]
mod unreachable {
    use super::*;

    #[test]
    fn ident() {
        #[irrefutable(unreachable)]
        let Some((ref a, ref mut b)) = Some(("a", "b"));
        assert_eq!(*a, "a");
        *b = "c";
        assert_eq!(*b, "c");
    }

    #[test]
    fn reference() {
        #[irrefutable(unreachable)]
        let Some((&a, &mut b)) = Some((&"a", &mut "b"));
        assert_eq!(a, "a");
        assert_eq!(b, "b");
    }

    #[test]
    fn r#struct() {
        #[irrefutable(unreachable)]
        let Some(Range { start, end }) = Some(Range { start: 0, end: 9 });
        assert_eq!(start, 0);
        assert_eq!(end, 9);
    }

    #[test]
    fn tuple() {
        #[irrefutable(unreachable)]
        let Some((a, b)) = Some(("a", "b"));
        assert_eq!(a, "a");
        assert_eq!(b, "b");
    }

    #[test]
    fn tuple_struct() {
        #[irrefutable(unreachable)]
        let Some(a) = Some("a");
        assert_eq!(a, "a");
    }
}
