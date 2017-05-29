# Generic RC

This repo explores a way to abstract over `Rc<T>` and `Arc<T>` using the
(limited?) capabilities of Rust as it exists at the time of writing this.

The idea is short and simple, that an extra page of description is unnecessary.
So please head over to [`src/lib.rs`](./src/lib.rs) to read the code and
comments.

**UPDATE:** /u/stevenportzer from reddit kindly [pointed out][] an alternate
method that uses a second indirection using traits to avoid making infinite
types without losing genericity. If the playground link over there fails to
work, [here is a copy][gist-link].

[pointed out]: https://www.reddit.com/r/rust/comments/6dz0xh/_/di6wvk9/
[gist-link]: https://gist.github.com/johncf/432cb5e7d166173c15bbe1507a46ac32

### Wait, why??

To let the user of a data structure library choose the type of reference
counting used internally without duplicating code (or macro gymnastics).

### Wait... WHY?

Because `Rc` is significantly faster than `Arc`. So if the user is running a
single-threaded program, she may not want all the features and over-heads that
`Arc` comes with.
