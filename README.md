# cgats

## Read, write, and manipulate [CGATS color files](http://www.colorwiki.com/wiki/CGATS.17_Text_File_Format)

This crate is a library intended to read, write, and transform CGATS color files and is currently in development. The included binary is a simple implementation of the library's API.

[Rust API Documentation](https://robeirne.github.io/cgats)

Or build the documentation yourself:

```sh
cargo doc --open
```

### The CGATS format

Here is a basic [CGATS color file](test_files/cgats1.tsv):

```tsv
CGATS.17
BEGIN_DATA_FORMAT
SAMPLE_ID	SAMPLE_NAME	CMYK_C	CMYK_M	CMYK_Y	CMYK_K
END_DATA_FORMAT
BEGIN_DATA
1	Cyan	100	0	0	0
2	Magenta	0	100	0	0
3	Yellow	0	0	100	0
4	Black	0	0	0	100
5	Blue	100	100	0	0
6	Red	0	100	100	0
7	Green	100	0	100	0
8	3cBlack	100	100	100	0
9	4cBlack	100	100	100	100
10	3cGray	50	40	40	0
11	1cGray	0	0	0	50
END_DATA
```

There are several more exmples in the [test_files](test_files) directory. The CGATS format is similar to TSV, but with a few additions. The `BEGIN_DATA_FORMAT`/`END_DATA_FORMAT` and `BEGIN_DATA`/`END_DATA` tags signal the way the measurements have been formatted.

### Binary Usage

```txt
USAGE:
    cgats [FILE]... [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <FILE>...    CGATS files

SUBCOMMANDS:
    average    Average 2 or more CGATS color files
    cat        Concatenate 2 or more CGATS color files
    help       Prints this message or the help of the given subcommand(s)
```

Print basic CGATS info to console:

```sh
cgats test_files/cgats0.txt
```

Average values of 3 CGATS files:

```sh
cgats average test_files/cgats0.tsv test_files/cgats1.tsv test_files/cgats2.tsv
```

### Binary Installation

First, you'll need to [download and install rust](https://rustup.rs). Then:

```sh
git clone https://github.com/robeirne/cgats
cd cgats
cargo build --release
cargo install
```

### TODO

- Add conversion functions and support for conversion to and from CXF/MXF
- Add smarter detection of DATA_FORMAT fields for better comparisons
- Add smoothing functions to correct measurement noise
- Add DeltaE integration with my [deltae](https://github.com/robeirne/deltae) crate