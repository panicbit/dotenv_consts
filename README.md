`dotenv_consts` is a macro that expands the environment variables in your dotfile to constanst.

```rust
mod env {
    ::dotenv_consts::dotenv_consts!();
}

fn main() {
    println!("{}", env::SOME_VAR);
}
```
