---
applyTo: '**'
---
# Code Style Guide

* DO NOT ADD ANY COMMENTS TO THE CODE.
* Avoid comments in the code. Keep narrative content in the chat, not in the files.
* Doccomments are allowed for public types and functions.
* Use meaningful variable names rather than shortened or cryptic names.
* Remove code rather than commenting it out.

## Examples of disallowed comments

```rust
  let mut x = 0; // Add this variable
```

```rust
  use std::collections::HashMap; // Importing HashMap
```

```rust
  fn test_x() {
        // x is derived from 1 + 3 = 4
        let x = 4;
  }
```

```rust
  fn add(a: i32, b: i32) -> i32 {
    // This function is used to calculate the sum of two numbers
    // It takes two parameters, a and b, both of type i32
    // It returns the sum of a and b, also of type i32
      a + b
  }
```