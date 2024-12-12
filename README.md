# RBC: CSV to CPA-005 Conversion Tool

There are 2 versions of this tool in this repository, one is built on the
Electron framework and can be run as a standalone desktop GUI application and
the other is a web version that requires a web browser. The Electron version is
really old and not maintained - it is **strongly recommended** to use the newer
Tauri version of this tool.

A CSV template is included in the project root directory. It is suggested you
open this CSV document in Excel. Replace the XXX's with real information.

This code implements the following specifications:
- ACH Direct Payments (PAP-PAD) Service Canadian Payments Association CPA-005
  Debit File Format Specifications
  [https://www.rbcroyalbank.com/ach/file-451771.pdf](https://www.rbcroyalbank.com/ach/file-451771.pdf)
- ACH Direct Deposits (PDS) Service Canadian Payments Association CPA-005 Credit
  File Format Specifications
  [https://www.rbcroyalbank.com/ach/file-451770.pdf](https://www.rbcroyalbank.com/ach/file-451770.pdf)

The CPA-005 specification is one of three specifications found at
[https://www.rbcroyalbank.com/ach/cid-212260.html](https://www.rbcroyalbank.com/ach/file-451770.pdf)

A downloaded copy of these specifications are located in the `spec/` directory
should these links break in the future.

The abundant number of versions and revisions is due to constant changing
customer requirements I was facing when developing this software.

## GUI Version (Tauri) (v3.0)

This version is the most functional one. It uses the same Rust backend that
powers the web version. I suggest using this version, unless there is a real
need to use the web version.

```bash
$ yarn install
$ yarn run tauri build
```

## Web Version (v2.0)

The web version is written in Rust and uses React for the browser UI. Unlike
traditional web servers, the compiled executable (the web server) is portable -
all the necessary HTML, CSS and JavaScript for the UI is stored in the
executable itself at compile time. This means once the executable has been
compiled, it is completely independent of this codebase and can be moved
anywhere.

We must first build the UI, as our web server executable will be loaded with the
UI's resulting build output.

```bash
$ cd web/ui
$ yarn install
$ yarn build-prod
$ cd ../rbc-rs
$ cargo build --release --bin web
```

To run the web server,

```bash
$ cargo run --bin web
```
or you can run directly with
```
$ ./target/release/web
```

You may move the executable at `target/release/web` to any other location.

### CLI

**This version has NOT been implemented.**

The web version also includes a CLI executable that allows for convenient
terminal use. It reuses most of the code from the web version. It does not
require the UI codebase to be built in advance. The CLI one can be built through

```
cargo build --release --bin cli
```

To run the CLI, 
```bash
$ cargo run --bin cli
```
or you can run directly with
```
$ ./target/release/cli
```

## Electron Version (v1.0)

Do not use the Electron version. It is kept in the codebase only as a archive.

