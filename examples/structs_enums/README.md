# structs_enums

A tiny example showing `Task` and `Command` and a basic CLI dispatcher.

Run the demo:

```powershell
cargo run -p structs_enums -- add "Write docs"
cargo run -p structs_enums -- list
cargo run -p structs_enums -- remove 1
```

This example is in-memory only (no persistence) and intended to illustrate `match`-based dispatch and enum handling.
