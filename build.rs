fn main() {
    build::main();
}

#[cfg(not(feature = "bundled"))]
mod build {
    use std::env;

    pub fn main() {
        if let Ok(dir) = env::var("LAME_LIB_DIR") {
            println!("cargo:rustc-link-search=native={}", dir);
        }

        if env::var("LAME_STATIC").is_ok() {
            println!("cargo:rustc-link-lib=static=mp3lame");
        } else {
            println!("cargo:rustc-link-lib=dylib=mp3lame");
        }
    }
}

#[cfg(feature = "bundled")]
mod build {
    extern crate gcc;
    extern crate target_info;

    use self::target_info::Target;

    pub fn main() {
        let mut config = gcc::Config::new();

        config.file("lame-3.99.5/libmp3lame/bitstream.c")
            .file("lame-3.99.5/libmp3lame/encoder.c")
            .file("lame-3.99.5/libmp3lame/fft.c")
            .file("lame-3.99.5/libmp3lame/gain_analysis.c")
            .file("lame-3.99.5/libmp3lame/id3tag.c")
            .file("lame-3.99.5/libmp3lame/lame.c")
            .file("lame-3.99.5/libmp3lame/mpglib_interface.c") //Perhaps remove
            .file("lame-3.99.5/libmp3lame/newmdct.c")
            .file("lame-3.99.5/libmp3lame/presets.c")
            .file("lame-3.99.5/libmp3lame/psymodel.c")
            .file("lame-3.99.5/libmp3lame/quantize_pvt.c")
            .file("lame-3.99.5/libmp3lame/quantize.c")
            .file("lame-3.99.5/libmp3lame/reservoir.c")
            .file("lame-3.99.5/libmp3lame/set_get.c")
            .file("lame-3.99.5/libmp3lame/tables.c")
            .file("lame-3.99.5/libmp3lame/takehiro.c")
            .file("lame-3.99.5/libmp3lame/util.c")
            .file("lame-3.99.5/libmp3lame/vbrquantize.c")
            .file("lame-3.99.5/libmp3lame/VbrTag.c")
            .file("lame-3.99.5/libmp3lame/version.c")
            .include("lame-3.99.5/include")
            .include("lame-3.99.5/libmp3lame")
            .define("HAVE_CONFIG_H", None)
            .define("PIC", None);

        if Target::os() == "windows" {
            config.define("TAKEHIRO_IEEE754_HACK", None)
                .define("FLOAT8", Some("float"))
                .define("REAL_IS_FLOAT", Some("1"))
                .define("BS_FORMAT", Some("BINARY"));
        }

        let os_config_dir = match Target::os() {
            "linux" => "linux",
            "macos" => "mac",
            "windows" => "win",
            os => panic!("unsupported os {}", os),
        };

        let arch_config_dir = match Target::arch() {
            "x86" => "ia32",
            "x86_64" => "x64",
            "arm" => "arm",
            arch => panic!("unsupported arch {}", arch),
        };

        config.include(format!("lame-config/{}/{}", os_config_dir, arch_config_dir));

        config.compile("libmp3lame.a");
    }
}