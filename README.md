# simple-logger

This is a simple logger to get the next results

```
[ TRACE ] message
[ DEBUG ] message
[ INFO  ] message
[ WARN  ] message
[ ERROR ] message
```

## Reference

the library responds to the next showing level table
| Level of request| TRACE | DEBUG | INFO | WARN | ERROR | OFF |
| :-------------- | :---- | :---- | :--- | :--- | :---- | :-- |
| TRACE | YES | NO | NO | NO | NO | NO |
| DEBUG | YES | YES | NO | NO | NO | NO |
| INFO | YES | YES | YES | NO | NO | NO |
| WARN | YES | YES | YES | YES | NO | NO |
| ERROR | YES | YES | YES | YES | YES | NO |

The logger functions uses rest parameters so you can set any number of parameters

## Usage

```toml
[dependencies]
logger = "0.1.0"
```

```rs
/// main.rs
use logger::{Levels, Logger};

fn main() {
    let logger = Logger::new(Levels::INFO); // Only INFO, WARN and ERROR log are alowed to print
    // Different ways to set string inside a vector
    logger.info(vec![
        String::from("Some"),
        "data: ".to_string(),
        format!("{}", 45),
    ])
}
// Output with INFO level
// [ INFO ] Some data 45
```

## License

MIT
