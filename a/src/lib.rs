pub trait Trait {}

#[cfg(test)]
fn _check_struct_is_trait() {
    fn check<T: Trait>() {}
    check::<b::Struct>();
}
