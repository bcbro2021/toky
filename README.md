# toky 😎👌🔥
A Simple natural language tokenizer built with rust..

## Installation
run this inside your project folder- <br />
```sh
git clone https://github.com/bcbro2021/toky.git
```
Inside Cargo.toml - <br />
```toml
[dependencies]
toky={path="./toky"}
```
## Getting Started
Let's create a simple token extraction program that lists all the words given..
### Basic Usage
```rust
use toky::*;

fn main() {
    // gets all tokens from the text
    let text: &str = "this is a test text!";
    let tokens = get_tokens(text);

    // gets only the words
    let string_tokens = &tokens["str"];

    // prints out an array of each word in the text
    println!("{:#?}",string_tokens);
}
```
