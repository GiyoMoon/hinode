# hinode
Utility to automatically switch between light and dark appearance modes on sunrise and sunset. Should work on any unix based system.

## Usage
```bash
Usage: hinode [OPTIONS] --latitude <LATITUDE> --longitude <LONGITUDE> --get-mode-cmd <GET_MODE_CMD> --light-mode-cmd <LIGHT_MODE_CMD> --dark-mode-cmd <DARK_MODE_CMD>

Options:
      --latitude <LATITUDE>
          Latitude of the location
      --longitude <LONGITUDE>
          Longitude of the location
      --get-mode-cmd <GET_MODE_CMD>
          Command to run to get the current mode Should return "dark" if dark mode and "light" if light mode
      --light-mode-cmd <LIGHT_MODE_CMD>
          Command to run when switching to light mode
      --dark-mode-cmd <DARK_MODE_CMD>
          Command to run when switching to dark mode
      --debug
          Whether to log debug information
  -h, --help
          Print help
  -V, --version
          Print version
```

### Example
```bash
cargo run -- \
  --latitude 46.9480 \
  --longitude 7.4474 \
  --get-mode-cmd "osascript -l JavaScript -e \"Application('System Events').appearancePreferences.darkMode() ? 'dark' : 'light'\"" \
  --light-mode-cmd "fish -c 'st light'" \
  --dark-mode-cmd "fish -c 'st dark'" \
  --debug
```
