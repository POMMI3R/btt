# B-Tree tester

btt is a simple B-Tree tester for Data Structure (CSE2010, 임종우) of Hanyang Univ.

Does not cover all cases but quite useful.

## How to use
### Clone

```
git clone https://github.com/POMMI3R/btt
```

### Install dependency

```
sudo apt install cargo
```

### Build

```
cargo build --release
```

### Run(Example)

Assuming that the directory structure is as follows(put your source code):

```
.
├── 2020068577.c
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    └── main.rs
```

run with command:

```
target/release/btt --file "2020068577.c" --order-start 10 --order-end 20 --size-start 10 --size-end 100
```

Then btt will test cases with order 10-20 and size 10-100.

### Command options

```
USAGE:
    btt [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --file <file>                  Source file  to test (Default: main.c)
        --order-end <order-end>        End    value of Order(Default:    100)
        --order-start <order-start>    Start  value of Order(Default:      2)
        --size-end <size-end>          End    size  of Input(Default:   1000)
        --size-start <size-start>      Start  size  of Input(Default:      1)
```