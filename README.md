<p align="center"><img width="128" height="128" src="./resources/logo.png" alt="logo"></p>

<p align="center">rofd </br> OFD parser and renderer library, in Rust.</p>
<p align="center"></p>


# Introduction

OFD (Open Form Document) is an open standard for electronic documents, which is widely used in China. Unlike PDF, which is a layout-based format, OFD is a semantic-based format, which means it stores the document structure and text information separately. This makes OFD documents more flexible and easier to edit than PDFs. OFD also supports more features than PDF, such as form filling and digital signatures.


# Plan

Current achivements:

- [x] learning
    - [x] Rust (basic syntax, types, traits, etc), notes [here](learning/notes.md)
    - [x] OFD spec (basic structure and elements)
- [x] parsing OFD files
- [x] drawing text, images, simple paths
- [x] rendering to png

Next steps: [TODOs.md](TODOs.md)


# Usage

To try it out, you can run the following command:

```bash
RUST_LOG=debug cargo test -- --nocapture
```

and

```bash
open target/out.png
```

# Logging

This library uses the `log` crate to record logs. To view log output, you need to initialize a logger implementation in your application. For example, using `env_logger`:


```rust
fn main() {
    env_logger::init();

    // your code...
}


# Reference projects

- [ofdrw](https://github.com/ofdrw/ofdrw)
- [ofd-parser](https://github.com/jyh2012/ofd-parser)
- [poppler](https://gitlab.freedesktop.org/poppler/poppler)

# License

This project is under the terms of the [MIT License](https://github.com/rofd/rofd/blob/main/LICENSE).