# Pomodoro Clock

Terminal Pomodoro timer application.

## Commands

```bash
# Add a timer to storage
pomodo add --name <name> --duration <minutes>

# Create a quick timer (runs immediately)
pomodo create --name <name> --duration <minutes>

# Run a stored timer by name
pomodo run --name <name>

# List all stored timers
pomodo list

# Remove a stored timer by name
pomodo remove --name <name>

# Remove all stored timers
pomodo nuke
```

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run --release -- <command>
```
