/// The abstract interface for a reference counted type.
pub trait RefCounted: Clone {
    type Target;

    fn new(obj: Self::Target) -> Self;
    fn strong_count(this: &Self) -> usize;
}

//The following impl is useless! See the test `it_works` for a detailed explanation and workaround.
//
//use std::rc::Rc;
//impl<T> RefCounted for Rc<T> {
//    type Target = T;
//
//    fn new(obj: T) -> Rc<T> {
//        Rc::new(obj)
//    }
//
//    fn strong_count(this: &Rc<T>) -> usize {
//        Rc::strong_count(this)
//    }
//}

#[cfg(test)]
mod tests {
    use super::RefCounted;

    use std::rc::Rc;
    use std::sync::Arc;

    struct Hello<T, RC>
    where RC: RefCounted<Target=Hello<T, RC>>
    {
        val: T,
        next: Option<RC>,
    }

    impl<T, RC> Hello<T, RC> where RC: RefCounted<Target=Hello<T, RC>> {
        fn new(val: T) -> Hello<T, RC> {
            Hello { val: val, next: None }
        }

        fn set_next(&mut self, val: T) {
            self.next = Some(RC::new(Hello::new(val)));
        }
    }

    #[test]
    fn it_works() {
        // The following expression fails to compile since Rust does not support recursive types
        // or, as the compiler calls it, "a type of infinite size"
        //
        //     let hello_bad = Hello<_, Rc<_>> { val: 0, next: None };
        //
        // Therefore we need to create newtypes which specifies both the wrapper type as well as
        // the target type. This means, for a given target type, we need to define newtypes which
        // implement RefCounted for every wrapper type we want to abstract over.
        //
        // This can be made easier by defining macros that generate such impls.

        // inferred since HelloArc is directly used
        let mut hello_arc = Hello { val: 0, next: None };
        hello_arc.next = Some(HelloArc::new(Hello { val: 1, next: None }));

        // using methods would need annotations
        let mut hello_rc = Hello::<_, HelloRc<_>>::new(0);
        hello_rc.set_next(1);
    }

    struct HelloRc<T>(Rc<Hello<T, HelloRc<T>>>);

    struct HelloArc<T>(Arc<Hello<T, HelloArc<T>>>);

    impl<T> RefCounted for HelloRc<T> {
        type Target = Hello<T, Self>;

        fn new(obj: Hello<T, Self>) -> Self {
            HelloRc(Rc::new(obj))
        }

        fn strong_count(this: &Self) -> usize {
            Rc::strong_count(&this.0)
        }
    }

    impl<T> Clone for HelloRc<T> {
        fn clone(&self) -> Self {
            HelloRc(self.0.clone())
        }
    }

    impl<T> RefCounted for HelloArc<T> {
        type Target = Hello<T, Self>;

        fn new(obj: Hello<T, Self>) -> Self {
            HelloArc(Arc::new(obj))
        }

        fn strong_count(this: &Self) -> usize {
            Arc::strong_count(&this.0)
        }
    }

    impl<T> Clone for HelloArc<T> {
        fn clone(&self) -> Self {
            HelloArc(self.0.clone())
        }
    }
}
