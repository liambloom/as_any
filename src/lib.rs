//! This is a very minimal crate that makes it easier to work
//! with traits that implement `Any` by allowing you to easily 
//! upcast to it.
//! 
//! # Example
//! 
//! ```
//! use core::any::Any;
//! use as_any_min::AsAny;
//! 
//! struct MyStruct;
//! trait MyTrait {}
//! impl MyTrait for MyStruct {}
//! 
//! /* Note that AsAny is automatically implemented for all 
//!     structs that implement Any, so there is no need to 
//!     implement it manually (in fact it won't compile if
//!     you try to) */
//! 
//! fn main() {
//!     // my_variable is now a trait object, which is the
//!     //  main use case for the AsAny trait.
//!     let my_variable: &dyn MyTrait = &MyStruct;
//! 
//!     let my_any_variable: &dyn Any = my_variable.as_any();
//! }
//! ```
//! 
//! # Without Using `AsAny`
//! 
//! Since rust doesn't (currently) have any built in way to
//! upcast from a trait object to another trait (such as `Any`), 
//! this won't compile.
//! 
//! ```compile_fail
//! use core::any::Any;
//! 
//! struct MyStruct;
//! trait MyTrait {}
//! impl MyTrait for MyStruct {}
//! 
//! fn main() {
//!     let my_variable: &dyn MyTrait = &MyStruct;
//! 
//!     let my_any_variable: &dyn Any = my_variable;
//! }
//! ```

#![no_std]
#![warn(missing_docs)]

use core::any::Any;

/// This trait allows anything that implements `Any` to be easily 
/// upcast to `Any`.
/// 
/// It is automatically implemented for all structs that implement
/// `Any`. This is useful because there isn't an automatic way to 
/// upcast a trait object.
pub trait AsAny {
    /// This returns the struct as a `&dyn Any`
    fn as_any(&self) -> &dyn Any;

    /// This returns the struct as a `&dyn mut Any`
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> AsAny for T
    where T: Any
{
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}