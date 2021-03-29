# midplay

`midplay` is a library for playing MIDI files from Rust. It can use MCI on Windows or a generic driver ([midir](https://github.com/Boddlnagg/midir)) on Windows, macOS and Linux.

## Usage

```rust
// MCI driver. (Windows only)
use midplay::native;

native::play_midi("data/CANYON.MID")?;
assert_eq!(true, native::is_midi_playing());

native::stop_midi()?;
assert_eq!(false, native::is_midi_playing());
```

``` rust
// Cross-platform driver.
use midplay::generic;

let mut out_ports = generic::get_ports()?;
for port in out_ports.iter() {
    println!("Index: {}  Name: {}", port.index, port.name);
}

let port = out_ports[0];
generic::play_midi("data/CANYON.MID", &port)?;
assert_eq!(true, generic::is_midi_playing());

generic::stop_midi()?;
assert_eq!(false, generic::is_midi_playing());
```

Run the example for each driver:

```sh
cargo run --example test_native
cargo run --example test_generic
```

## Credits

Most of the code for the generic driver was taken from [rusthesia](https://github.com/gin66/rusthesia).

## License

MIT.
