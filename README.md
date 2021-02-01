# World Clock

This is a small command-line application that shows the current time in multiple time zones.

## Config

Which timezones are displayed is controlled by a config file at `~/.config/worldclock.toml`. 

Example config file:

```toml
# Local clock
[[clocks]]
name = "💻"

# At home
[[clocks]]
name = "🏠"
tz = "Europe/Berlin"

# Costa Rica
[[clocks]]
name = "🌴"
tz = "America/Costa_Rica"

# New York (EST)
[[clocks]]
name = "🗽"
tz = "America/New_York"
```

Each `[[clock]]` will display one line with the current time. If you don't specify the timezone with `tz` it'll use the current time.

Otherwise you can specify the timezone. For valid timezones you can have a look at `timedatectl list-timezones`.

Each clock can have a optional name, that will be displayed instead of the timezone name.

## Output

Example output:

```
 💻  20:03:33
 🏠  20:03:33
 🌴  13:03:33
 🗽  14:03:33
```
