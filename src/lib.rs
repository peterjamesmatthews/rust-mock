//! An example rust library that easily mocks an interface for unit testing with the
//! [`mockall`](https://crates.io/crates/mockall) crate.
//!
//! An important [`Application`](struct.Application.html) method
//! [`cool_algorithm`](struct.Application.html#method.cool_algorithm) needs to be unit tested.
//!
//! [`cool_algorithm`](struct.Application.html#method.cool_algorithm) relies on an external service
//! and for whatever reason, that external service cannot be used during unit testing.
//!
//! To remedy this, we define the [`I32Calculator`](trait.I32Calculator.html) trait that
//! [`ExternalI32Calculator`](struct.ExternalI32Calculator.html) implements and add a
//! [`i32_calculator`](struct.Application.html#structfield.i32_calculator) field to our
//! [`Application`](struct.Application.html) that is [`I32Calculator`](trait.I32Calculator.html).
//!
//! To enable mocking of [`I32Calculator`](trait.I32Calculator.html), we add the
//! `#[cfg_attr(test, mockall::automock)]` attribute to the trait's definition.
//!
//! In our unit test, we then create a mock object that is
//! [`I32Calculator`](trait.I32Calculator.html) and set up expectations and return values for the
//! methods will be called during the
//! [`cool_algorithm`](struct.Application.html#method.cool_algorithm) call.
//!
//! ```rust
//! #[cfg(test)]
//! mod tests {
//!   use super::*;
//!   use mockall::predicate;
//!
//!   #[test]
//!   fn cool_algorithm_does_nothing() {
//!     let number = 100;
//!
//!     // mock object that is I32Calculator
//!     let mut mock_i32_calculator = MockI32Calculator::new();
//!
//!     // set up our expectations
//!     mock_i32_calculator
//!       .expect_add()
//!       .times(1)
//!       .with(predicate::eq(number), predicate::eq(0))
//!       .return_const(number);
//!
//!     mock_i32_calculator
//!       .expect_subtract()
//!       .times(1)
//!       .with(predicate::eq(number), predicate::eq(0))
//!       .return_const(number);
//!
//!     mock_i32_calculator
//!       .expect_multiply()
//!       .times(1)
//!       .with(predicate::eq(number), predicate::eq(1))
//!       .return_const(number);
//!
//!     mock_i32_calculator
//!       .expect_divide()
//!       .times(1)
//!       .with(predicate::eq(number), predicate::eq(1))
//!       .return_const(number);
//!
//!     // create our application with our mock calculator
//!     let app = Application {
//!       i32_calculator: Box::new(mock_i32_calculator),
//!     };
//!
//!     // run our unit test of the cool_algorithm
//!     assert_eq!(app.cool_algorithm(number), number);
//!   }
//! }
//! ```

/// Mockable trait that a client for an external service would implement.
#[cfg_attr(test, mockall::automock)]
pub trait I32Calculator {
  /// Returns the sum of `x` and `y`.
  fn add(&self, x: i32, y: i32) -> i32;
  /// Returns the difference of `x` and `y`.
  fn subtract(&self, x: i32, y: i32) -> i32;
  /// Returns the product of `x` and `y`.
  fn multiply(&self, x: i32, y: i32) -> i32;
  /// Returns the quotient of `x` and `y`.
  fn divide(&self, x: i32, y: i32) -> i32;
}

/// Toy client implementation of [`I32Calculator`](trait.I32Calculator.html) that panics when called.
///
/// This would be used by the real application, but never during unit testing.
pub struct ExternalI32Calculator;
impl I32Calculator for ExternalI32Calculator {
  fn add(&self, _x: i32, _y: i32) -> i32 {
    panic!("Can't call this in unit tests!")
  }

  fn subtract(&self, _x: i32, _y: i32) -> i32 {
    panic!("Can't call this in unit tests!")
  }

  fn multiply(&self, _x: i32, _y: i32) -> i32 {
    panic!("Can't call this in unit tests!")
  }

  fn divide(&self, _x: i32, _y: i32) -> i32 {
    panic!("Can't call this in unit tests!")
  }
}

/// struct that has an [`I32Calculator`](trait.I32Calculator.html)
/// [`i32_calculator`](struct.Application.html#structfield.i32_calculator) field.
pub struct Application {
  pub i32_calculator: Box<dyn I32Calculator>,
}

impl Application {
  /// An important bit of application logic that makes use of the `I32Calculator` interface.
  ///
  /// This will be unit tested.
  pub fn cool_algorithm(&self, x: i32) -> i32 {
    let mut output = x;

    output = self.i32_calculator.add(output, 0);
    output = self.i32_calculator.subtract(output, 0);
    output = self.i32_calculator.multiply(output, 1);
    output = self.i32_calculator.divide(output, 1);

    output
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use mockall::predicate;

  #[test]
  fn cool_algorithm_does_nothing() {
    let number = 100;

    // mock object that is I32Calculator
    let mut mock_i32_calculator = MockI32Calculator::new();

    // set up our expectations
    mock_i32_calculator
      .expect_add()
      .times(1)
      .with(predicate::eq(number), predicate::eq(0))
      .return_const(number);

    mock_i32_calculator
      .expect_subtract()
      .times(1)
      .with(predicate::eq(number), predicate::eq(0))
      .return_const(number);

    mock_i32_calculator
      .expect_multiply()
      .times(1)
      .with(predicate::eq(number), predicate::eq(1))
      .return_const(number);

    mock_i32_calculator
      .expect_divide()
      .times(1)
      .with(predicate::eq(number), predicate::eq(1))
      .return_const(number);

    // create our application with our mock calculator
    let app = Application {
      i32_calculator: Box::new(mock_i32_calculator),
    };

    // run our unit test of the cool_algorithm
    assert_eq!(app.cool_algorithm(number), number);
  }
}
