# hinode
MacOS utility to automatically switch between light and dark appearance modes on sunrise and sunset.

## Usage
```bash
Usage: hinode [OPTIONS] --latitude <LATITUDE> --longitude <LONGITUDE> --light-mode-cmd <LIGHT_MODE_CMD> --dark-mode-cmd <DARK_MODE_CMD>

Options:
      --latitude <LATITUDE>              Latitude of the location
      --longitude <LONGITUDE>            Longitude of the location
      --light-mode-cmd <LIGHT_MODE_CMD>  Command to run when switching to light mode
      --dark-mode-cmd <DARK_MODE_CMD>    Command to run when switching to dark mode
      --debug                            Whether to log debug information
  -h, --help                             Print help
  -V, --version                          Print version
```

### Example
```bash
cargo run -- \
  --latitude 46.9480 \
  --longitude 7.4474 \
  --light-mode-cmd "fish -c 'st light'" \
  --dark-mode-cmd "fish -c 'st dark'" \
  --debug
```
