[![Workflow Status](https://github.com/liambloom/as_any/workflows/main/badge.svg)](https://github.com/liambloom/as_any/actions?query=workflow%3A%22main%22)

# as_any_min

This is a very minimal crate that makes it easier to work
with traits that implement `Any` by allowing you to easily
upcast to it.

## Example

```rust
use core::any::Any;
use as_any_min::AsAny;

struct MyStruct;
trait MyTrait {}
impl MyTrait for MyStruct {}

/* Note that AsAny is automatically implemented for all
    structs that implement Any, so there is no need to
    implement it manually (in fact it won't compile if
    you try to) */

fn main() {
    // my_variable is now a trait object, which is the
    //  main use case for the AsAny trait.
    let my_variable: &dyn MyTrait = &MyStruct;

    let my_any_variable: &dyn Any = my_variable.as_any();
}
```

## Without Using `AsAny`

Since rust doesn't (currently) have any built in way to
upcast from a trait object to another trait (such as `Any`),
this won't compile.

```rust
use core::any::Any;

struct MyStruct;
trait MyTrait {}
impl MyTrait for MyStruct {}

fn main() {
    let my_variable: &dyn MyTrait = &MyStruct;

    let my_any_variable: &dyn Any = my_variable;
}
```

License: MIT
