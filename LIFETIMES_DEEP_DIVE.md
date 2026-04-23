# Lifetimes in Rust — Deep Dive

A comprehensive guide to understanding lifetimes, one of Rust's most powerful (and confusing) features.

---

## Table of Contents
1. [The Problem Lifetimes Solve](#the-problem-lifetimes-solve)
2. [Lifetime Syntax](#lifetime-syntax)
3. [Lifetime Inference (Elision)](#lifetime-inference-elision)
4. [Lifetimes in Structs](#lifetimes-in-structs)
5. [Common Lifetime Patterns](#common-lifetime-patterns)
6. [Lifetime Bounds](#lifetime-bounds)
7. [Visual Examples](#visual-examples)
8. [Interview Q&A](#interview-qa)

---

## The Problem Lifetimes Solve

### Dangling References (The Rust Way)

In C/C++, you can create a **dangling pointer** — a reference to freed memory:

```c
// C code — DANGEROUS
int* get_int() {
    int x = 5;
    return &x;  // ❌ x is freed when function returns!
}
// Calling code uses freed memory → undefined behavior, crashes
```

In Rust, **this is impossible at compile time**:

```rust
// Rust code — WON'T COMPILE
fn get_int() -> &i32 {
    let x = 5;
    &x  // ❌ ERROR: cannot return reference to local variable
}
```

**Why Rust prevents this:**
- Rust sees that `x` is a local variable
- `x` is dropped when the function returns
- A reference to `x` would become dangling
- Compilation fails; no runtime crash possible

### Lifetimes Make This Explicit

Lifetimes let you express "how long a reference is valid":

```rust
fn get_ref<'a>(input: &'a str) -> &'a str {
    // "I return a reference valid for 'a"
    // "as long as input is valid"
    input
}

let s = String::from("hello");
let r = get_ref(&s);  // 'a = lifetime of s
println!("{}", r);    // ✅ s is still alive
// s is dropped here
```

---

## Lifetime Syntax

### Basic Notation

A lifetime is written as `'name` (apostrophe + identifier):

```rust
&'a str          // reference to str, valid for lifetime 'a
&'a String       // reference to String, valid for lifetime 'a
&'static str     // reference valid for entire program (static lifetime)
```

### In Functions

**Explicit lifetime parameter:**

```rust
fn process<'a>(text: &'a str) -> &'a str {
    // 'a is a generic parameter (like T in generics)
    // Tied together: input and output both use 'a
    text
}

// Call it:
let s = "hello";
let result = process(s);  // 'a is inferred as the lifetime of s
```

**Multiple lifetimes:**

```rust
fn merge<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
    // s1 has lifetime 'a, s2 has lifetime 'b
    // Returns reference with lifetime 'a (tied to s1)
    s1
}
```

**Lifetime bounds:**

```rust
fn compare<'a, 'b: 'a>(s1: &'a str, s2: &'b str) -> bool {
    // 'b: 'a means "'b outlives 'a" (b lives longer)
    s1.len() < s2.len()
}
```

---

## Lifetime Inference (Elision)

Rust can infer many lifetimes automatically. These rules are called **lifetime elision**.

### Rule 1: Each Parameter Gets Its Own Lifetime

```rust
fn example(s1: &str, s2: &str) { }

// Rust expands to:
fn example<'a, 'b>(s1: &'a str, s2: &'b str) { }
// Each reference gets a unique lifetime
```

**Consequence:** If you return a reference, it's ambiguous which parameter it comes from:

```rust
fn problematic(s1: &str, s2: &str) -> &str {
    if s1.len() > s2.len() { s1 } else { s2 }
    // ❌ ERROR: ambiguous which lifetime to return
    // Does it live as long as s1 or s2?
}

// MUST be explicit:
fn correct<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
    // ✅ OK: both inputs and output share 'a
}
```

### Rule 2: Single Input → Output Gets Same Lifetime

```rust
fn first_word(s: &str) -> &str {
    &s.split_whitespace().next().unwrap_or("")
}

// Rust expands to:
fn first_word<'a>(s: &'a str) -> &'a str { ... }
// Output lifetime matches input — no ambiguity
```

This is **lifetime elision in action** — you don't write the `<'a>` part.

### Rule 3: Methods with `&self`

```rust
impl Parser {
    pub fn get_buffer(&self) -> &str {
        &self.buffer
    }
}

// Rust expands to:
impl Parser {
    pub fn get_buffer<'a>(&'a self) -> &'a str {
        &self.buffer
    }
}
// Returns reference valid as long as self exists
```

**When to use elision:**
- ✅ Single input reference, output reference
- ✅ Method with `&self` returning a reference
- ❌ Multiple inputs, unclear which output comes from

**When to be explicit:**
- Multiple input references
- Return different lifetime than input
- Struct fields holding references

---

## Lifetimes in Structs

### The Core Principle

**Structs can hold references, but must promise the references stay valid.**

Syntax: `struct Name<'a> { field: &'a Type }`

```rust
pub struct SearchResult<'a> {
    pub pattern: String,
    pub matching_lines: Vec<&'a str>,  // "references valid for 'a"
}

// This means: "a SearchResult can hold references to data
//              that lives for at least 'a"
```

### Building a Lifetime-Aware Struct

```rust
// 1. Define the struct with lifetime parameter
#[derive(Serialize)]
pub struct SearchResult<'a> {
    pub pattern: String,
    pub matching_lines: Vec<&'a str>,
}

// 2. Function that returns the struct
pub fn search_pattern<'a>(content: &'a str, pattern: &str) -> SearchResult<'a> {
    let mut matching_lines: Vec<&'a str> = Vec::new();
    
    for line in content.lines() {
        if line.contains(pattern) {
            matching_lines.push(line);  // line: &'a str (from content)
        }
    }
    
    SearchResult {
        pattern: pattern.to_string(),
        matching_lines,
    }
}

// 3. Use it
let content = String::from("Rust\nJava\nRust");
let result = search_pattern(&content, "Rust");
println!("{:?}", result.matching_lines);  // ✅ OK: content still alive

// result cannot outlive content:
let result;
{
    let content = String::from("temp");
    result = search_pattern(&content, "temp");
}  // ❌ ERROR: content dropped here, but result holds refs to it
```

### Why This Matters

**Comparison: owned vs borrowed**

```rust
// ❌ Owned version (allocates memory):
pub struct SearchResultOwned {
    pub matching_lines: Vec<String>,  // each line is cloned
}

let content = "Line1\nLine2\nLine3".repeat(1000);  // 1MB
let result = search_pattern_owned(&content, "Line1");
// Creates copies of all matching lines in memory

// ✅ Borrowed version (zero allocation):
pub struct SearchResult<'a> {
    pub matching_lines: Vec<&'a str>,  // just references
}

let content = "Line1\nLine2\nLine3".repeat(1000);  // 1MB
let result = search_pattern(&content, "Line1");
// No extra memory! Just pointers to original content
```

**Efficiency gains:**
- Owned: O(n) memory for n matches
- Borrowed: O(1) memory for n matches (pointers only)
- Borrowed: Faster (no cloning)

### Multiple Lifetime Parameters

```rust
pub struct Pair<'a, 'b> {
    first: &'a str,
    second: &'b str,  // different lifetimes!
}

// Usage:
let s1 = "hello";
{
    let s2 = "world";
    let pair = Pair { first: s1, second: s2 };
    println!("{} {}", pair.first, pair.second);  // ✅ OK
}  // s2 dropped here

// pair.second is now invalid, but:
let pair: Pair;  // won't compile without a valid s2
```

---

## Common Lifetime Patterns

### Pattern 1: Static Lifetime (`'static`)

References valid for **the entire program lifetime**:

```rust
let global: &'static str = "I live forever";

// Function returning 'static reference:
fn get_program_name() -> &'static str {
    "MyApp"  // string literal, lives forever
}

// Using 'static as a bound:
fn takes_static(s: &'static str) {
    // Requires a reference that lives for the entire program
}

takes_static("hello");  // ✅ OK: string literal is 'static
let s = String::from("world");
takes_static(&s);  // ❌ ERROR: s is not 'static
```

### Pattern 2: Returning the Shorter Lifetime

```rust
fn shortest<'a, 'b: 'a>(s1: &'a str, s2: &'b str) -> &'a str {
    // 'b: 'a means "b outlives a"
    // So we can safely return &'a (the shorter one)
    s1
}
```

### Pattern 3: Lifetime in Impl Blocks

```rust
pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { input }
    }
    
    pub fn parse(&self) -> &'a str {
        // Returns reference with original 'a lifetime
        self.input
    }
}

// Usage:
let data = "parse me";
let parser = Parser::new(data);
let result = parser.parse();  // result: &'a str
```

### Pattern 4: Higher-Ranked Trait Bounds (HRTB)

Advanced pattern for complex cases:

```rust
// For now, recognize the syntax:
fn process<F>(f: F)
where
    F: for<'a> Fn(&'a str) -> usize,
{
    // f can work with references of ANY lifetime
    // "for any lifetime 'a, f takes &'a str"
}
```

---

## Lifetime Bounds

### The Syntax

`'a: 'b` means **"'a outlives 'b"** or **"'a lives at least as long as 'b"**

```rust
// 'a: 'b means 'a >= 'b in duration
fn bound_example<'a, 'b: 'a>(x: &'b str, y: &'a str) -> &'a str {
    // 'b must be at least as long as 'a
    // Safe to return y (it lives longer)
    y
}
```

### Lifetime Bounds with Structs

```rust
pub struct Cache<'a> {
    data: &'a str,
}

impl<'a> Cache<'a> {
    // Method requires another lifetime to outlive 'a
    pub fn compare<'b: 'a>(&self, other: &'b str) -> bool {
        self.data.len() < other.len()
    }
}
```

---

## Visual Examples

### Example 1: String Lifetime Visualization

```rust
fn main() {
    let s = String::from("hello");  // s is created
    // ───────────────────────────── 's lifetime starts
    
    let r: &str = &s;              // r borrows s
    // ───────────────────────────── 'r lifetime: borrow of s
    
    println!("{}", r);              // r used here
    
    // ───────────────────────────── 'r ends (last use)
    
    // ───────────────────────────── 's ends (dropped)
}
```

### Example 2: Multiple References

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}

let s1 = "hello";      // 'a >= duration of s1
let s2 = "rust";       // 'a >= duration of s2
let result = longest(s1, s2);
// 'a = min(lifetime of s1, lifetime of s2)

println!("{}", result);

// All drop in reverse order
```

### Example 3: Struct Holding References

```rust
struct SearchResult<'a> {
    lines: Vec<&'a str>,
}

fn search<'a>(content: &'a str) -> SearchResult<'a> {
    // 'a = lifetime of content
    SearchResult { lines: vec![&content[..1]] }
}

let content = String::from("data");
let result = search(&content);
println!("{:?}", result);  // ✅ OK: content still alive

// ❌ This would NOT work:
let result;
{
    let content = String::from("temp");
    result = search(&content);
    // content dropped here, but result holds a reference!
}
// Compilation error caught!
```

---

## Interview Q&A

### Q1: Explain lifetimes in one sentence.

**Answer:** "Lifetimes are labels that track how long references are valid; they prevent use-after-free bugs at compile time."

### Q2: Why do structs need lifetime parameters?

**Answer:** "A struct holding a reference must specify how long that reference is valid. The lifetime ensures the reference can't outlive the data it points to."

Example:
```rust
struct Parser<'a> { input: &'a str }
// "This parser holds a reference to data that lives for 'a"
```

### Q3: What's the difference between `&String` and `&'a String`?

**Answer:** "They're semantically the same (both are borrowed references), but `&'a String` **explicitly names** the lifetime. Use explicit lifetimes when you need to relate multiple references."

```rust
fn process(s: &String) -> &String { s }  // Elision hides lifetime

fn process<'a>(s: &'a String) -> &'a str {  // Explicit
    &s[..]
}
```

### Q4: When would you use `Vec<&'a str>` instead of `Vec<String>`?

**Answer:** "When you want to **borrow references** instead of owning copies. Benefits:
- Zero allocation (just pointers)
- Faster (no cloning)
- Data changes reflected in references

Tradeoff: References can't outlive the source data."

```rust
// Efficient: borrow references
pub struct SearchResult<'a> {
    lines: Vec<&'a str>,  // O(1) memory
}

// Flexible but costly: own data
pub struct SearchResult {
    lines: Vec<String>,   // O(n) memory
}
```

### Q5: What does `'a: 'b` mean?

**Answer:** "'a outlives 'b" or "'a lives at least as long as 'b'".

```rust
fn example<'a, 'b: 'a>(x: &'b str, y: &'a str) {
    // 'b >= 'a in duration
    // It's safe to use y where x could be used
}
```

### Q6: Can you have a struct without a lifetime parameter?

**Answer:** "Yes, unless it holds references. Structs owning data don't need lifetimes."

```rust
pub struct Parser {
    input: String,  // ✅ No lifetime needed (owns data)
}

pub struct ParserRef<'a> {
    input: &'a str,  // ✅ Lifetime needed (borrows data)
}
```

### Q7: What's the `'static` lifetime?

**Answer:** "`'static` means the reference is valid for the **entire program duration**. String literals and global constants are `'static`."

```rust
let s: &'static str = "I live forever";

// Function requiring 'static:
fn requires_static(s: &'static str) { }

requires_static("literal");  // ✅ OK
let s = String::from("owned");
requires_static(&s);         // ❌ ERROR: not 'static
```

### Q8: What's the difference between lifetime inference and explicit lifetimes?

**Answer:** "Inference: Rust automatically figures out lifetimes (often). Explicit: You write them in the code.

- Inferred: `fn process(s: &str) -> &str` — single parameter/return
- Explicit: `fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str` — multiple parameters

Use explicit when Rust can't infer unambiguously."

---

## Summary: Key Takeaways

1. **Lifetimes prevent use-after-free** — the biggest memory safety bug
2. **`'a` is a label** for a period of time during which a reference is valid
3. **Lifetimes are erased at runtime** — zero cost
4. **Lifetime elision** lets you skip explicit syntax in simple cases
5. **Structs holding references** must have lifetime parameters
6. **`Vec<&'a str>`** is more efficient than `Vec<String>` (no allocation)
7. **Lifetime bounds** (`'a: 'b`) relate multiple lifetimes
8. **`'static`** means valid for the entire program

---

## Practice Exercises

Try these to deepen understanding:

### Exercise 1: Fix the Compilation Error

```rust
fn get_greeting() -> &str {
    let s = String::from("hello");
    &s  // ❌ ERROR: fix this
}

// Solution: Return owned data or take input
fn get_greeting() -> String {
    String::from("hello")  // ✅ Return owned String
}
```

### Exercise 2: Understand Lifetime Relationships

```rust
fn process<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
    s1  // ✅ Why is this OK?
}

// Answer: 'a and 'b are independent lifetimes
// Returning &'a str (from s1) is always safe
```

### Exercise 3: Struct Lifetime Practice

```rust
pub struct Cache<'a> {
    data: &'a str,
}

// What happens if you try:
let cache;
{
    let s = String::from("temp");
    cache = Cache { data: &s };
}
println!("{}", cache.data);  // ❌ Why error?

// Answer: s dropped at }, but cache holds a reference to s
```

---

## Resources

- [The Rust Book — Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [Rust by Example — Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
- [Lifetime Elimination Rules](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision)
- [Higher-Ranked Trait Bounds](https://doc.rust-lang.org/nomicon/hrtb.html)
