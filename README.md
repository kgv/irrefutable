# Irrefutable

[![Build Status](https://travis-ci.org/kgv/irrefutable.svg?branch=master)](https://travis-ci.org/kgv/irrefutable)
[![Build Status](https://ci.appveyor.com/api/projects/status/github/kgv/irrefutable?svg=true)](https://ci.appveyor.com/project/kgv/irrefutable)
[![Crates](https://img.shields.io/crates/v/irrefutable.svg)](https://crates.io/crates/irrefutable)
[![Docs](https://docs.rs/irrefutable/badge.svg)](https://docs.rs/irrefutable)
[![License](https://img.shields.io/crates/l/irrefutable)](#license)

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
