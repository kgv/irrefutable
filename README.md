# Irrefutable

Procedural macro to convert a refutable let expression to an irrefutable.

## Examples

unreachable:

```rust
#![feature(proc_macro_hygiene)]

use irrefutable::irrefutable;

#[irrefutable(unreachable)]
let Some((a, b)) = Some(("a", "b"));
```

expands to:

```rust
let (a, b) = if let Some((a, b)) = Some(("a", "b")) {
    (a, b)
} else {
    unreachable!();
};
```

return:

```rust
#![feature(proc_macro_hygiene)]

use irrefutable::irrefutable;

#[irrefutable(return)]
let Some((a, b)) = Some(("a", "b"));
```

expands to:

```rust
let (a, b) = if let Some((a, b)) = Some(("a", "b")) {
    (a, b)
} else {
    return;
};
```
