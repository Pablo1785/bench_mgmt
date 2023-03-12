What I miss in my day-to-day from Rust:
- immutability by default AKA mutability is explicit, e.x. fn compute(&mut data: Vec<i64>) -> i64; VS. def compute(data: List[int]) -> int; VS. const compute = (data: number[]): number; conclusion in Rust - this function is a bit weird VS. conclusion in other langs - I need to check the func body to see if it's weird
- no easy opt-out of the type system, e.x. func handle(data: interface{}): (*interface{}, err); void handle(Object data); def handle(data); const handle = (data) => {};
- nulls don't ruin your day; nullability is handled at compile time via Option<>; no need for clumsy afterthought null-safety syntax; language was designed to be like this from the start so no issues with dependencies on that front
- built-in unified tooling AKA rustup, cargo add, cargo fmt, cargo check, cargo fix, cargo bench, cargo test, rustdoc
- expression-based syntax AKA inline if, loop, while, for, match
- widespread pattern matching AKA if let, let else, while let, match, but also inside function signatures, regular variable declarations and many other places; works for structs, enums, slices/arrays, strings - you can count on it wherever you need it;
- enums are sum types - which works beautifully with pattern matching
- great performance is always there - for most applications you get it essentialy for free; similar to Golang in that regard
- macros metaprogramming - if there is a language-level issue some smart person will come around to make a handy syntax-sugar macro for patching it until a language update provides a stable solution (which in programming languages can take years), e.x. async_trait, thiserror
- iterator API - on the surface very similar to Java Stream API but flows much smoother thanks to expression-based syntax and syntax sugar offered by Rust, as well as much more type inference that the compiler can do thanks to the stricter type system; [paradoxically by being much stricter with your typing you have to write less type annotations]

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
