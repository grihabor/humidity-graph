humidity-graph
==============

Rust project for the _Arduino Mega 2560_.

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## Connect the device

1. Connect to HC-05 via bluetooth
2.
```bash
sudo rfcomm connect /dev/rfcomm0 98:D3:71
``` 
3.
```bash
sudo chmod 777 /dev/rfcomm0
```
4.
```bash
docker-compose up -d
```
5.
```bash
clickhouse client -q "$(cat schema.sql)"
```
6.
```bash
python receiver.py
```

## License
Licensed under MIT license

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
