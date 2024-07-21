Mini-grep
=========

Implementation of a simple grep like the
[exercise of the Rust Book](https://doc.rust-lang.org/book/ch12-00-an-io-project.html).

**Please don't use it or use it at your risks.**
You can use [ripgrep](https://github.com/BurntSushi/ripgrep).

[![Version](https://img.shields.io/badge/Version-v0.5.4-blue.svg)]()
[![Rust](https://img.shields.io/badge/Language-Rust2021-blue.svg)]()
[![LGPL3.0 License](https://img.shields.io/badge/License-LGPL%20v3.0-green.svg)](https://www.gnu.org/licenses/lgpl-3.0.html)
![Build](https://img.shields.io/badge/Build-Cargo%201.79.0%20-graen.svg)

## Table of contents

* [Technologies](#tech-stack)
* [Setup](#setup)

## Tech Stack

Requirements:

- Rust 2021
- Cargo 1.79.0

## Setup

You can download the project and build it with Meson.

### Clone the project:

```bash
git clone https://github.com/Xyphenore/mini-grep.git
```

### Build it:

```shell
cargo build
```

### Run it

To search a pattern in a file

```shell
cargo run -- Rust resources/example.txt
```

To ignore case sensitivity

Unix

```shell
IGNORE_CASE=1; cargo run -- Rust resources/example.txt
```

Windows (cmd)

```shell
set IGNORE_CASE=1 && cargo run -- Rust resources/example.txt
```

Windows (PowerShell)

```shell
$env:IGNORE_CASE = "1"; cargo run -- Rust resources/example.txt
```

### Generate the documentation

```shell
cargo doc
```

Open the [target/doc/mini_grep/index.html](target/doc/mini_grep/index.html) in the directory
[target/doc/web_server](target/doc/mini_grep).

```shell
cd target/doc/web_server
```
