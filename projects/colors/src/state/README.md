Rust source files in this directory are intended for defining structs that represent the application's mutable state data.

The files are included as modules in an automatically generated file called state.rs (stored in the project/.tmp/state/ directory). Structs within these files following the filename.rs -> FilenameState pattern will be publicly exported as state::FilenameState.