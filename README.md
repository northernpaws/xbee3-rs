# xbee3-rs

This crate provides an interface for communicating with XBee 3 devices/modules, both on embedded devices that require `no_std`, and on non-embedded devices with the standard library available.

The base module for the crate provides all the common types and traits, and then platform support (i.e. stm32 via [Embassy](https://embassy.dev)) are provided as modules. To use the library, you should use one of the platform-specific modules to initialize an XBee device using the interfaces available for that platform (i.e. UART, Serial over USB, etc.). 

## `no_std` Usage

```toml
# Example with the stm32 platform integration.
[dependencies.xbee3-rs]
# Signals to the library to NOT
# include the standard library.
default-features = false
# The `stm32` feature enables defmt,
# no_std, and Embassy stm32 helpers.
features = ["stm32"] 
```
