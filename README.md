# This is **cole_cli** (working name), a compact CLI app written in Rust

## I started writing this as an excuse to learn Rust - because Rust is neat

```console
A compact, jack-of-all trades command line tool.

Usage: cole_cli.exe <COMMAND>

Commands:
  print      Prints contents of file to stdout
  num-lines  Counts the number of lines in file
  env-vars   Provides stats about working environment
  generate   TODO implement the Generate command
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

---

As of version 0.2.0 the tool is capable of a few commands, `print` and `num-lines`
Both require an input file passed in. The default value won't suffice.

### `print`

```console
Prints contents of file to stdout

Usage: cole_cli.exe print [OPTIONS]

Options:
  -i, --input <FILE(S)>  File to be used as input [default: -]
  -n, --nums             Displays line numbers
  -h, --help             Print help
```

### `num-lines`

```console
Counts the number of lines in file

Usage: cole_cli.exe num-lines [OPTIONS]

Options:
  -i, --input <INPUT>  file to be used as input [default: -]
  -h, --help           Print help
```

$${\color{red}The \space work \space continues...}$$
