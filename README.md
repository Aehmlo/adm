# `adm`

[![Build Status](https://travis-ci.com/Aehmlo/adm.svg?branch=master)](https://travis-ci.com/Aehmlo/adm)

Short for "Alex's Device Manager," `adm` is a set of tools for smart home automation.

Currently, only LIFX bulbs are supported, and only the CLI is useful, but this will change as my own needs evolve.

If you'd like to see support added for another device type, feel free to [file an issue](https://github.com/Aehmlo/adm/issues/new) or [open a pull request](https://github.com/Aehmlo/adm/compare)!

## Usage Example

To begin, you'll need to set up your devices in `~/.adm/config.toml` (we'll provide a utility for this some day, but for now, you'll have to do it by hand).

Currently, the only supported devices are LIFX bulbs, so you'll have to [go get a LIFX API secret key](https://api.developer.lifx.com/docs/authentication).

```toml
lifx-secret = "your secret here"

[[devices]]
type = "lifx" # LIFX is the only supported device type right now
selector = "label:Downstairs Lamp" # https://api.developer.lifx.com/docs/selectors
name = "Downstairs Lamp" # User-friendly name, to be used as the primary device identifier
alternatives = ["Lamp"] # Other ways you might want to address the device (optional)
```

Note that the `names` and `alternatives` fields are case-insensitive (so you'll be able to use `turn lamp on` instead of `turn Lamp on` for the above device, for example).

Add a `[[devices]]` entry for each device you want to control, then save the configuration file.

Go ahead and test out the configuration using something like `adm turn <device> off` (see `adm help turn` for more information), and if it behaves as expected, you're good to go! (Note that no output means the change was successful.)

Once you've gotten the command-line interface working, you can do some low-tech scheduling! I'll probably introduce another way to do this in the future, but for now, we'll use `cron` to automate turning on the lamp at 6 PM every day. Add the following to your crontab, and you're off to the races!

```crontab
0 18 * * * /path/to/adm turn on lamp >/dev/null 2>&1
```

## Contributing

Contributions are welcome! As mentioned above, feel free to contribute however you can, whether it be through filing an issue, opening a pull request, or whatever else.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as below, without any additional terms or conditions.

## License

Licensed under [the Apache License, Version 2.0](https://opensource.org/licenses/Apache-2.0) or [the MIT License](https://opensource.org/licenses/MIT), at your
option.