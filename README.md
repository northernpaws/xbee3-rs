# xbee3-rs

This crate provides an interface for communicating with XBee 3 devices/modules, both on embedded devices that require `no_std`, and on non-embedded devices with the standard library available.

The base module for the crate provides all the common types and traits, and then platform support (i.e. stm32 via [Embassy](https://embassy.dev)) are provided as modules. To use the library, you should use one of the platform-specific modules to initialize an XBee device using the interfaces available for that platform (i.e. UART, Serial over USB, etc.). 

## Usage

### `no_std` Embassy Usage

The `embassy` module and associated features (i.e. `stm32`) provide transports for communicating with the XBee devices over MCU peripherals (i.e. UART), synchronized using `embassy-sync` channels.

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

### 'tokio' Usage

The `tokio` module provides a base transport that uses Tokio's sync modules for synchronizing API frame packet transmission and reception with `PubSubChannel`s.

```toml
# Example using Tokio for asynchronous messaging
# and serialport for xbee serial access.
[dependencies.xbee3-rs]
features = ["tokio", "serialport"] 

```

## Examples

 * `serialport` - Uses `tokio` for syncronization and `serialport` for communicating with USB serial devices.
 * `stm32f401re` - Uses `embassy` and `embassy-sync` for communicating with an XBee radio from an embedded device.


## Troubleshooting

### RTS Configuration

By default the Xbee 3 modules enable CTS flow control on pin DIO7. Unless RTS on the host is connected,this appears to cause a problem where the module goes into a pseudo-sleep mode and doesn't process API packets until the hosts's RTS signal is asserted.