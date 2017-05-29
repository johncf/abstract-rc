# Generic RC

This repo explores a way to abstract over `Rc<T>` and `Arc<T>` using the
(limited?) capabilities of Rust as it exists at the time of writing this.

The idea is short and simple, that an extra page of description is unnecessary.
So please head over to [`src/lib.rs`](./src/lib.rs) to read the code and
comments.

### Wait, why??

To let the user of a data structure library choose the type of reference
counting used internally without duplicating code (or macro gymnastics).

### Wait... WHY?

Because `Rc` is waay faster than `Arc`. So if the user is running a single-
threaded program, she may not want all the features and over-heads that `Arc`
comes with.
