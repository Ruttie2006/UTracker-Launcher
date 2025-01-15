# UTracker Launcher

A simple utility to launch the archipelago universal tracker.

## Configuration

All values except the `host` will be prompted.
However, you might want to have some defaults that you do not want to enter every time.
To do this, you can create a `config.toml` file in the same location as the executable.

A config file should look something like this:
```toml
# All values here are optional, don't add them and they will be asked when you run the program
location = 'C:\Program Files\Archipelago'
port = 26950
host = 'archipelago.gg'
```