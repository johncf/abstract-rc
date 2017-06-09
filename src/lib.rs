use std::ops::Deref;

/// The abstract interface for a reference counted type.
pub trait MutablePtr: Deref {
    fn new(obj: Self::Target) -> Self;
    fn make_mut(this: &mut Self) -> &mut Self::Target;
}

// The following impl isn't useful! See `it_works` for a detailed explanation and workaround.
//
//     impl<T: Clone> MutablePtr for std::rc::Rc<T> { ... }

#[cfg(test)]
mod tests {
    use super::MutablePtr;

    use std::ops::Deref;
    use std::rc::Rc;
    use std::sync::Arc;

    struct Hello<T, RC>
    where RC: MutablePtr<Target=Hello<T, RC>>
    {
        val: T,
        next: Option<RC>,
    }

    impl<T, RC> Clone for Hello<T, RC>
    where T: Clone,
          RC: MutablePtr<Target=Hello<T, RC>> + Clone,
    {
        fn clone(&self) -> Self {
            Hello {
                val: self.val.clone(),
                next: self.next.clone(),
            }
        }
    }

    impl<T, RC> Hello<T, RC> where RC: MutablePtr<Target=Hello<T, RC>> {
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
        // implement MutablePtr for every wrapper type we want to abstract over.
        //
        // This can be made easier by defining macros that generate such impls.

        // inferred since HelloArc is directly used
        let mut hello_arc = Hello { val: 0, next: None };
        hello_arc.next = Some(HelloArc::new(Hello { val: 1, next: None }));

        // using methods would need annotations
        let mut hello_rc = Hello::<_, HelloRc<_>>::new(0);
        hello_rc.set_next(1);

        struct NotClone;

        // using methods would need annotations
        let mut hello_box = Hello::<_, HelloBox<_>>::new(NotClone);
        hello_box.set_next(NotClone);
    }

    struct HelloRc<T: Clone>(Rc<Hello<T, HelloRc<T>>>);

    struct HelloArc<T: Clone>(Arc<Hello<T, HelloArc<T>>>);

    struct HelloBox<T>(Box<Hello<T, HelloBox<T>>>);

    impl<T: Clone> Deref for HelloRc<T> {
        type Target = Hello<T, Self>;

        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }

    impl<T: Clone> MutablePtr for HelloRc<T> {
        fn new(obj: Hello<T, Self>) -> Self {
            HelloRc(Rc::new(obj))
        }

        fn make_mut(this: &mut Self) -> &mut Self::Target {
            Rc::make_mut(&mut this.0)
        }
    }

    impl<T: Clone> Clone for HelloRc<T> {
        fn clone(&self) -> Self {
            HelloRc(self.0.clone())
        }
    }

    impl<T: Clone> Deref for HelloArc<T> {
        type Target = Hello<T, Self>;

        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }

    impl<T: Clone> MutablePtr for HelloArc<T> {
        fn new(obj: Hello<T, Self>) -> Self {
            HelloArc(Arc::new(obj))
        }

        fn make_mut(this: &mut Self) -> &mut Self::Target {
            Arc::make_mut(&mut this.0)
        }
    }

    impl<T: Clone> Clone for HelloArc<T> {
        fn clone(&self) -> Self {
            HelloArc(self.0.clone())
        }
    }

    impl<T> Deref for HelloBox<T> {
        type Target = Hello<T, Self>;

        fn deref(&self) -> &Self::Target {
            &*self.0
        }
    }

    impl<T> MutablePtr for HelloBox<T> {
        fn new(obj: Hello<T, Self>) -> Self {
            HelloBox(Box::new(obj))
        }

        fn make_mut(this: &mut Self) -> &mut Self::Target {
            &mut *this.0
        }
    }

    impl<T: Clone> Clone for HelloBox<T> {
        fn clone(&self) -> Self {
            HelloBox(self.0.clone())
        }
    }
}
