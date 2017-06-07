# lame-sys

Rust FFI bindings to libmp3lame.

[Documentation](https://docs.rs/lame-sys/)

## Notes

A bundled version of LAME 3.99.5 can be built by selecting the feature `bundled`. In this case this crate becomes licensed under the LGPL.

To link statically to `libmp3lame` set `LAME_STATIC` to any value. In this case this crate becomes licensed under the LGPL.

You can set the `LAME_LIB_DIR` to point to a directory containing the LAME library.
