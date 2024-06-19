# safenv

A thread-safe alternative to the `std::env` module

## Examples

```rust
use safenv as env;

let key = "KEY";
env::set_var(key, "VALUE");
assert_eq!(env::var(key), Ok("VALUE".to_string()));
```

**Made with 💗 by Juliette Cordor**
