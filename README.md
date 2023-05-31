# RUSPATCH

This program extracts and compiles the unsafe function in the rust source code into a dynamic link library, and modifies the call to the unsafe function in the source code into a call to a dynamic link library function

## Build

```shell
cargo build
```

## Usage

```shell
Usage: ruspatch [OPTIONS] <DIRECTORY>

Arguments:
  <DIRECTORY>  rust project directory

Options:
  -i          inject thread
  -h, --help  Print help
```

After successful conversion, a dynamic link library file will be generated in the project directory.

Put the binary file compiled by the modified source code and the dynamic link library file into the same directory for execution.

## Limitations

1. Rust projects can only be compiled to executables (no src/lib.rs).
2. All use statements in the source code are at the beginning of the file.
3. All unsafe functions in the source code do not have the same name.
4. Generic unsafe functions are not supported.
5. If the parameter and return type of the unsafe function is not pre-imported, it may need to be imported manually.

## Cloud environment

The profile is available on https://www.cloudlab.us/p/Rpatch/ruspatch-env.
