# Irrefutable

Procedural macro to convert a refutable let expression to an irrefutable.

## Examples

panic:

```rust
#![feature(proc_macro_hygiene)]

use irrefutable::irrefutable;

#[irrefutable(panic("The cause."))]
let Some((a, b)) = Some(("a", "b"));
```

expands to:

```rust
let (a, b) = if let Some((a, b)) = Some(("a", "b")) {
    (a, b)
} else {
    panic!("The cause.");
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
