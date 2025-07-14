# clipass_rs - Library for masked CLI input reading

### Description
```
This library provides the basic functionality for masking the input reading 
of a console application. By setting up the required needs, the custom prompt
won't reveal the input unless it is allowed to do so. The input can either 
directly be accessed or hashed in (currently) sha256 and md5.
```

### Usage
- Import the crate
```rust
use clipass_rs::CliPass;
```

- Initialize clipass_rs
```rust
let mut session = CliPass::new();
```

#### Settings
- Specify a custom prompt label:
```rust
pub fn set_prompt_label(&mut self, custom_label: &str)
```

- Ignore the default prompt label and only read the input:
```rust
pub fn set_no_label(&mut self)
```

- Per default the arrow down key can be used to unmask the input (arrow up to mask it again), this can be prevented with:
```rust
pub fn set_no_visibility(&mut self)
```

- The mask token (default: '*') can be updated to the prefered one with:
```rust
pub fn set_prompt_mask_token(&mut self, custom_token: char)
```

- To launch the prompt after setup:
```rust
pub fn launch_prompt(&mut self) -> io::Result<String>
```

### Example
```rust
use std::io;

use clipass_rs::CliPass;

fn main() {
    const HASH_256: &str = "405f42005704da932ea8a4ad1f1e8c26e751af0316caa1e7e4bef4af4e2d93fe"; // correctpassword

    let mut session = CliPass::new();
    session.set_prompt_mask_token('#');
    session.set_no_label(); // We don't want to display any prompt label
    session.launch_prompt()?;
            
    // Compare the required hash against the hashed password provided by the user
    assert_eq!(session.hash_sha256_internal(), HASH_256);
}

/*
* Output:
* -------
*   ############### <-- "invalidpassword"
*
*   thread 'main' panicked at src/bin/main.rs:56:9:
*   assertion `left == right` failed
*    left: "5bd7f6cf3b61dd672341c7cc2baafd3e960b01ffef4509bb631b4b267e85b444"
*   right: "405f42005704da932ea8a4ad1f1e8c26e751af0316caa1e7e4bef4af4e2d93fe"
*/
```

#### Full Examples can be found [here](https://github.com/f42h/clipass_rs/tree/master/examples)

## License
This project is published under the [MIT](https://github.com/f42h/clipass_rs/blob/master/LICENSE) license.