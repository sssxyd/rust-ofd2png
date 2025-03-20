# 介绍

本项目初始代码从 [rofd](https://github.com/hualet/rofd) fork 而来

# 编译

## ubuntu
1. `sudo apt-get install libcairo2-dev`


# 用法


# 说明

This project is organized into the following directories and files:

- `src/`: source code.
- `src/lib.rs`: the main library crate.
- `src/document.rs`: document parsing and rendering.
- `src/page.rs`: page parsing and rendering.
- `src/render.rs`: rendering to Cairo surface.
- `src/types.rs`: types used in OFD spec.
- `src/elements.rs`: OFD elements.
- `src/ofd.rs`: OFD file parsing.
- `learning/`: learning notes and examples.
- `resources/`: resources, such as the logo.
- `LICENSE`: license file.
- `Cargo.toml`: cargo configuration file.
- `README.md`: this readme file.


# 参考
- [rofd](https://github.com/hualet/rofd)
- [ofdrw](https://github.com/ofdrw/ofdrw)
- [ofd-parser](https://github.com/jyh2012/ofd-parser)
- [poppler](https://gitlab.freedesktop.org/poppler/poppler)