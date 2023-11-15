# Rust Mock

An example rust library that easily mocks an interface for unit testing with the
[`mockall`](https://crates.io/crates/mockall) crate.

An important `Application` method `cool_algorithm` needs to be unit tested.

`cool_algorithm` relies on an external service and for whatever reason, that external service cannot
be used during unit testing.

To remedy this, we define the `I32Calculator` trait:
https://github.com/peterjamesmatthews/rust-mock/blob/8e6303efc4035f87d4bfd7204484634838393ce9/src/lib.rs#L74-L83

And implement it for external service's client `ExternalI32Calculator`:
https://github.com/peterjamesmatthews/rust-mock/blob/8e6303efc4035f87d4bfd7204484634838393ce9/src/lib.rs#L89-L105

We then declare an `i32_calculator` field in our `Application` that is `I32Calculator`:
https://github.com/peterjamesmatthews/rust-mock/blob/8e6303efc4035f87d4bfd7204484634838393ce9/src/lib.rs#L109-L111

To enable mocking of `I32Calculator`, we add the `#[cfg_attr(test, mockall::automock)]` attribute to
the trait's definition:
https://github.com/peterjamesmatthews/rust-mock/blob/8e6303efc4035f87d4bfd7204484634838393ce9/src/lib.rs#L73-L74

In our unit test, we then create a mock object that is `I32Calculator` and set up expectations and
return values for the methods will be called during the `cool_algorithm` call:
https://github.com/peterjamesmatthews/rust-mock/blob/8e6303efc4035f87d4bfd7204484634838393ce9/src/lib.rs#L129-L174
