<p align="center"><img width="128" height="128" src="./resources/logo.png" alt="logo"></p>

<p align="center">rofd </br> OFD parser and renderer library, in Rust.</p>
<p align="center"></p>


## Introduction

OFD (Open Form Document) is an open standard for electronic documents, which is widely used in China. Unlike PDF, which is a layout-based format, OFD is a semantic-based format, which means it stores the document structure and text information separately. This makes OFD documents more flexible and easier to edit than PDFs. OFD also supports more features than PDF, such as form filling and digital signatures.


# plan

I'm still learning Rust and OFD spec by creating this project, so the lib is not first priority at the moment, I just want the parser and renderer to work.

To try the current achivements, you can run the following command:

```bash
cargo test
```

and

```bash
open target/out.png
```

next steps: [TODOs.md](TODOs.md)

# reference projects

- [ofdrw](https://github.com/ofdrw/ofdrw)
- [ofd-parser](https://github.com/jyh2012/ofd-parser)
- [poppler](https://gitlab.freedesktop.org/poppler/poppler)

# License

This project is under the terms of the [MIT License](https://github.com/rofd/rofd/blob/main/LICENSE).