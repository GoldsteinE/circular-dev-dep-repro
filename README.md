```shell
$ cargo c --all-targets
    Checking a v0.1.0 (/tmp/tmp.oxMZ9gYMte/a)
    Checking b v0.1.0 (/tmp/tmp.oxMZ9gYMte/b)
error[E0277]: the trait bound `Struct: Trait` is not satisfied
 --> a/src/lib.rs:6:13
  |
6 |     check::<b::Struct>();
  |             ^^^^^^^^^ the trait `Trait` is not implemented for `Struct`
  |
note: required by a bound in `check`
 --> a/src/lib.rs:5:17
  |
5 |     fn check<T: Trait>() {}
  |                 ^^^^^ required by this bound in `check`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `a` (lib test) due to previous error
```
