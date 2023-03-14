Install Rust at https://rustup.rs/

src/playground contains examples. Recommended order:
1. Variables
2. Types
3. Traits
4. Errors
5. Iterators

Run exmaple code with ```cargo run --bin <variables | types | errors | ...>```

Start by walking through and playing around in the playground at your own pace.

Afterwards complete the service/implement/* methods marked with todo!().

If you're feeling decently comfortable with Rust already you can start with the implementations.

The presentation part is adapted from https://github.com/0atman/noboilerplate/blob/main/scripts/4-rust-impatient.md as well as https://fasterthanli.me/articles/a-half-hour-to-learn-rust

If something is unclear or unexplained refer to the above, or https://doc.rust-lang.org/stable/book/


# Zen of Python

*Beautiful is better than ugly.* 
```rust
let Point { x, y } = my_point;
``` 
VS.
```python
x, y = my_point.x, my_point.y
```
*Explicit is better than implicit.* 
```rust
pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err>
where
    F: FromStr,
```
VS.
```python
class int(x, base=10)  # no errors? :o
```

*Readability counts.*
- That's why Rust has types

*Errors should never pass silently.*

*Unless explicitly silenced.*
```rust
pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err>
where
    F: FromStr,
```
VS.
```python
class int(x, base=10)  # no errors? :o
```


*In the face of ambiguity, refuse the temptation to guess.*
```rust
// Following will not compile:
let empty = 0;
if empty {
    println!("Not gonna guess what you mean...")
} else {
    println!("...it's just 4 additional characters: '== 0'")
}
```
VS.
```python
empty = 0
if empty:
    print("Not here")
else:
    print("I guess 0 means false :D (or maybe it was None, who knows? I sure don't)")
```
*There should be one-- and preferably only one --obvious way to do it.*
```bash
cargo add handy_library
```
VS.
```bash
pip install handy_library
conda add handy_library
poetry add handy_library
```

*Now is better than never.*
```rust
// I need to handle this one if I want to safely use my values
pub fn parse<F>(&self) -> Result<F, <F as FromStr>::Err>
where
    F: FromStr,
```
VS.
```python
class int(x, base=10)  # no errors :D
```

*Although never is often better than *right* now.*
```rust
let num: i64 = data.parse().expect("I'm just trying things out, the linter will remind me to handle this before Code Review");
```
VS.
```python
num = int(data)  # lgtm, I'm sure the data will be fine most of the time
```
