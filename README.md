# shq

```sh
shq: shell queue
  simple string queue for command line

USAGE:
    shq [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --source <source>    Queue source file (default: /path/to/cache_dir/shq/defalut)

SUBCOMMANDS:
    clear    clear queue
    help     Prints this message or the help of the given subcommand(s)
    pop      pop from queue
    push     push to queue
    show     show queue
```

## Install
バイナリ配布手順をまだ作って無いです

```sh
$ git clone https://github.com/xztaityozx/shq
$ cd shq
$ cargo build --release
$ ln -s $PWD/target/release/shq ~/.local/bin/
```
