# platform-switch

Namespace facades for switching between platforms.

## Motivation
Sometimes code needs to be shared between a contrainted platform
(such as an embedded system) and a target that has a more fully
featured environment. This crate provides a series of module wrappers
that can be used in combination with cargo manifest feature definitions
to switch between options at compile time.

## Example
Using `thiserror` with `defmt` enabled on an embedded system:

```toml
[features]
default = ["std"]
std = ["platform-switch/std_error"]
mcu = ["platform-switch/core_error", "platform-switch/defmt"] # Requires nightly
```

```rust
use platform_switch::log as log;
use platform_switch::thiserror as thiserror;

#[derive(thiserror::Error, Debug)]
enum ExampleError {
    #[error("Error #1")]
    Error1,
    #[error("Error #2")]
    Error2,
}

fn error_logger() {
    log::trace!("Trace");
    log::debug!("Debug");
    log::info!("Info");
    log::warn!("Warn");
    log::error!("Error");

    log::error!("Error: {}", ExampleError::Error1);
    log::error!("Error: {}", ExampleError::Error2);
}
```

### Stable Toolchain
If a stable toolchain is required, `thiserror` can be disabled with the following features and attributes:
```toml
[features]
default = ["std"]
std = ["thiserror", "platform-switch/std_error"]
mcu = ["platform-switch/defmt"]
thiserror = []
```

```rust
use platform_switch::log as log;
use platform_switch::thiserror as thiserror;

#[derive(Debug)]
#[cfg_attr(feature = "thiserror", derive(thiserror::Error))]
enum ExampleError {
    #[cfg_attr(feature = "thiserror", error("Error #1"))]
    Error1,

    #[cfg_attr(feature = "thiserror", error("Error #2"))]
    Error2,
}
```

# License

[Mozilla Public License Version 2.0](https://www.mozilla.org/en-US/MPL/2.0/)
