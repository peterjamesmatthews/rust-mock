# Rust Mock

An example rust library that easily mocks an interface for unit testing with the
[`mockall`](https://crates.io/crates/mockall) crate.

An important `Application` method `cool_algorithm` needs to be unit tested.

`cool_algorithm` relies on an external service and for whatever reason, that external service cannot
be used during unit testing.

To remedy this, we define the `I32Calculator` trait that `ExternalI32Calculator` implements and add
a `i32_calculator` field to our `Application` that is `I32Calculator`.

To enable mocking of `I32Calculator`, we add the `#[cfg_attr(test, mockall::automock)]` attribute to
the trait's definition.

In our unit test, we then create a mock object that is `I32Calculator` and set up expectations and
return values for the methods will be called during the `cool_algorithm` call.

_TODO: add code examples_
