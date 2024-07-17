# Basics

This document consists of notes taken while reading the book [The Rust Programming Language](https://doc.rust-lang.org/book). Unnamed chapter references are references to this book.

## Syntax

#### Types

A variable's type can be inferred or it can be defined as having a certain type.


```Rust
let n1 = 20;
let n2: i32 = 22;
let n3: f32 = 24.68;
let s1: &str = "hello, types";
```

#### Immutability

Rust defaults to immutable variables.


```Rust
let i: i32 = 63110;

println!("{}", i);
```

    63110



```Rust
i = 3; // error: cannot assign twice to immutable variable
```


    [E0384] Error: cannot assign twice to immutable variable `i`

       ╭─[command_10:1:1]

       │

     1 │ i = 3; // error: cannot assign twice to immutable variable

       │ ──┬──  

       │   ╰──── cannot assign twice to immutable variable

       │ 

       │ Note: You can change an existing variable to mutable like: `let mut x = x;`

    ───╯



```Rust
let mut b: f32 = 2.63110001;
b = 3.00000001;

println!("{}", b);
```

    3



```Rust
let s: &str = "foo";

println!("{}", s);
```

    foo


#### `new`

The `new` in `let s = String::new();` is a function that returns a new **instance** of a String.

The `::` indicates `new` is an _associated function_  of the `String` type.

> An _associated function_ is a function that’s implemented on a type, in this case `String`.

-- The Rust Programming Language, Chapter 2: _Programming a Guessing Game_

#### Functions


```Rust
fn main() {
    println!("hello, rust");
}
```

A function named `main` has special meaning and will be picked up as the entry point.

#### Ranges

Ranges are defined using `start..end` notation.

Without any additional operators, the range is inclusive on the lower and exclusive on the upper bound, i.e., $start \le x \lt end$.


```Rust
for i in 1..3 {
    println!("{i}");
}
```

    1
    2





    ()



A `=` at the end position, as in `1..=100` makes the range inclusive on the lower and upper bounds.


```Rust
for i in 1..=3 {
    println!("{i}");
}
```

    1
    2
    3





    ()



For a more exhaustive listing, see [The Rust Reference: Range expressions](https://doc.rust-lang.org/1.79.0/reference/expressions/range-expr.html).

#### Macros

Macros typically end with a `!` as in `println!()`.

##### `println!()`

`println!()` takes a format string. This string can contain named variables as in `"{named_variable}` or placeholders in the form of empty curly brackets, as in `"{}"`.

The placeholders will be replaced  by the remaining arguments in the same order as they appeared. These can be variables but also expressions.


```Rust
let named_variable = "I am a named variable";

println!("{named_variable}");
println!("{}", named_variable);
println!("{}", "I am a placeholder");
println!("{}", (3 + 3) / 2 == 3);
```

    I am a named variable
    I am a named variable
    I am a placeholder
    true


#### References

The `&` in `read_line(&mut input)` is a **reference**.

The Book describes references as _"a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times."_ (Chapter 2)

References are also immutable by default. A mutable reference is defined as `&mut reference_name`.

### `match`

Chapter 2 gives the following example when building the guessing game:


```Rust
use std::cmp::Ordering;

let guess = 5;
let secret_number = 8;

match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

    Too small!





    ()



Here, `std::cmp::Ordering` is an enum, and the `cmp` method on the `guess` integer returns a variant of this enum. This allows them to be matched in the match expression above.

## Types

### `std::result::Result`

`io::stdin().read_line(&mut input)` does not return the string itself. It returns a `Result` value.

`Result` is an enum with the variants `Ok` and `Err`.

The `Ok` variant contains the actual value. In the case of `read_line`, the number of bytes in the input it read.

The `Err` variant means the operation failed and contains information on how or why it failed.

`Result` has a method called `expect`.

When the `Result` instance is an `Err` variant, `expect` causes the program to crash and displays the error message passed to it. This error can be [handled](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html). 

When the `Result` intance is an `Ok` variant, `expect` returns the return value that `Ok` is holding.

In an example from Chapter 2, the following is shown:


```Rust
let mut guess = String::new();
let guess: u32 = guess.trim().parse().expect("Please type a number!"); // panics
```

    thread '<unnamed>' panicked at src/lib.rs:101:39:
    Please type a number!: ParseIntError { kind: Empty }
    stack backtrace:
       0: rust_begin_unwind
                 at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/std/src/panicking.rs:652:5
       1: core::panicking::panic_fmt
                 at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/panicking.rs:72:14
       2: core::result::unwrap_failed
                 at /rustc/129f3b9964af4d4a709d1383930ade12dfe7c081/library/core/src/result.rs:1654:5
       3: <unknown>
       4: <unknown>
       5: evcxr::runtime::Runtime::run_loop
       6: evcxr::runtime::runtime_hook
       7: evcxr_jupyter::main
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.


In the code above, `parse` returns a `Result` type, which has the `expect` method. 

If `guess` before shadowing were a string containing a number character, such as `"2"` or `"84"`, it would successfully get [parsed](https://doc.rust-lang.org/std/primitive.str.html#method.parse). Because it doesn't, the returned `Result` is an `Err` variant. Due to this, the `expect` call causes a panic.

When a valid number is passed instead, the `parse` call will return an `Ok` variant of `Result`, and when `expect` is called on it, instead of causing a panic, it will return the parsed number.

### `std::cmp::Ordering`

`std::cmp::Ordering` is an enum with the variants `Less`, `Greater` and `Equal`, representing the three possible outcomes in a comparison.

Many types have a `cmp` method that perform a comparison betwene the caller and the argument. These methods return a variant of `Ordering`.

The following example uses a [`match` expression](https://doc.rust-lang.org/book/ch06-02-match.html) to demonstrate this.


```Rust
fn compare(x: i32, y: i32) {
    match x.cmp(&y) {
            Ordering::Less => println!("Smaller"),
            Ordering::Greater => println!("Bigger"),
            Ordering::Equal => println!("Equal"),
    }
}

compare(1, 2);
compare(2, 1);
compare(3, 3);
```

    Smaller
    Bigger
    Equal


### `std::io::Stdin`

To read user input, one can use `read_line` from `std::io::Stdin`:

```rust
let mut input = String::new();

io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
```

The argument to `read_line()`, in this case `&mut input`, is the string to store user input in.

`read_line` does not overwrite the present contents of this string. It appends a new line every time it is called.

## Libraries

Some parts of the standard library are always imported by default, called the [prelude](https://doc.rust-lang.org/std/prelude/index.html). Other parts need to be imported with `use`:


```Rust
use std::io;
```

If a `use` statement is not included, one can still call the contents of `std::io` by using, for example, `std::io::stdin().read_line(&mut input)`.

The Book describes `std::io::stdin` as returning _"an instance of `std::io::Stdin, which is a type that represents a handle to the standard input for your terminal." (Chapter 2)

## Compilation & Tooling

### Compilation

Compilation without the Cargo build tool uses `rustc`:

```sh
rustc main.rs
./main
Hello, world!
```

> Rust is an _ahead-of-time compiled_ language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

-- The Rust Programming Language, Chapter 1.2: _Hello, World!_

### Cargo

Cargo's primary roles are to compile the project source code and manage dependencies.

#### Project creation

```sh
cargo new hello_cargo
cd hello_cargo
```

Upon creating a project, cargo will setup the following file structure:

```
.
├── .git
├── .gitignore
├── Cargo.toml
└── src
    └── main.rs
```

There are additional options for project creation. See `cargo new --help` for an overview.

#### `Cargo.toml`

Cargo projects are configured through a `Cargo.toml` at the project root. Upon project creation, it has the following contents:

```toml
[package]
name = "hello2"
version = "0.1.0"
edition = "2021"

[dependencies]
```

For a reference containing all options, see [the manifest format](https://doc.rust-lang.org/cargo/reference/manifest.html).

#### Checking

The `cargo check` command will analyze the code and determine if it compiles, but without producing any executables.

It's a faster alternative to always building just to know if the code can compile.

#### Building

A project can be built with `cargo build`. This will create a `target/debug` directory containing the build artifacts.

This will also produce a `Cargo.lock` file at the project root.

To produce an optimized release executable, use `cargo build --release`.

A release build is slower and will create executables in `target/release`.

#### Running

The project can be executed with `cargo run`. If it has not been built yet, it will be.

It is aware of code modifications, so it will not build again if nothing has changed.

#### Adding dependencies

Dependencies can be added manually to the `Cargo.toml` file:

```toml
[package]
name = "hello2"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8.5"
```

The same can be achieved with `cargo add rand@0.8.5`

> The specifier `0.8.5` is actually shorthand for `^0.8.5`, which means any version that is at least 0.8.5 but below 0.9.0.

-- The Rust Programming Language, Chapter 2: _Programming a Guessing Game_

A `Cargo.lock` file also gets created containing the versions for the whole dependency tree. This ensures that builds are reproducible by locking dependency versions. This means versions of dependencies will not change unless explicitly upgraded to.

This explicit update is done through `cargo update`, which in this case will _"only look for versions greater than 0.8.5 and less than 0.9.0"_ (Chapter 2).

Unless the `Cargo.toml` file is changed to state `rand = "0.9.0"`, Cargo will keep ignoring this version and all that come after it.

#### Building documentation

`cargo doc --open` builds the documentation for the project and its dependencies locally and opens them in a browser.

This is very useful to get a collated set of the documentation relevant to the project available offline.



### rustup

Primarily used to install the `rustc` compiler and the Rust toolchain, rustup has several other features.

#### Displaying documentation

`rustup doc` opens a very comprehensive index of documentation available offline, including:

- the standard library's documentation
- The Rust Programming Language, also known as "The Rust Book"
- The Rust Reference
- Rust By Example
- the `rustc`, Cargo, Rustdoc and Clippy books
- an index of error codes.

### Clippy

Clippy is a linter. It can be installed with `rustup component add clippy` and used with `cargo clippy`, which also appears as as the `cargo-clippy` executable.

### `rustfmt`

`rustfmt` formats code. It can be used as `cargo fmt` to format a whole Cargo project, or on an individual file as  `rustfmt <file>`.

### `rustfix`

`rustfix` fixes some errors flagged by the compiler. It's used on the whole project with `cargo fix`.

### rust-analyzer

`rust-analyzer` is a language server for Rust.

It can be installed with `rustup component add rust-analyzer` and also requires the standard library sources. To obtain these, run `rustup component add rust-src`.

A compatible editor configured for it is also needed. For Neovim, see [nvim-lspconfig's server configuration](https://github.com/neovim/nvim-lspconfig/blob/master/doc/server_configurations.md#rust_analyzer).

### See also

- [The Cargo Book](https://doc.rust-lang.org/cargo/index.html)
- [Clippy documentation](https://github.com/rust-lang/rust-clippy)
- [rustfmt documentation](https://github.com/rust-lang/rustfmt)
- [rust-analyzer homepage](https://rust-analyzer.github.io/)

## References

1. [The Rust Programming Language](https://doc.rust-lang.org/1.79.0/book/), version 1.79.0 assuming 1.76.0 (released 2024-02-08)
2. `rustc`, version 1.79.0 (129f3b996 2024-06-10)
3. [The Rust Reference](https://doc.rust-lang.org/1.79.0/reference/), Rust 1.79.0
4. [Rust standard library documentation](https://doc.rust-lang.org/1.79.0/std/index.html), version 1.79.0 (129f3b996 2024-06-10)
