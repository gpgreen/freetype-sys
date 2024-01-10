use std::env;

fn add_sources(build: &mut cc::Build, root: &str, files: &[&str]) {
    let root = std::path::Path::new(root);
    build.files(files.iter().map(|src| {
        let mut p = root.join(src);
        p.set_extension("c");
        p
    }));

    build.include(root);
}

fn main() {
    let target = env::var("TARGET").unwrap();
    if !target.contains("android")
        && pkg_config::Config::new()
            .atleast_version("24.3.18")
            .find("freetype2")
            .is_ok()
    {
        return;
    }

    // for the Rust esp32 IDF configuration, we need to specify the target compilers
    if target == "xtensa-esp32-espidf" {
        env::set_var("CXX", "xtensa-esp32-elf-g++");
        env::set_var("CC", "xtensa-esp32-elf-gcc");
        env::set_var("CFLAGS", "-mlongcalls");
        env::set_var("CXXFLAGS", "-mlongcalls");
    } else if target == "xtensa-esp32s3-espidf" {
        env::set_var("CXX", "xtensa-esp32s3-elf-g++");
        env::set_var("CC", "xtensa-esp32s3-elf-gcc");
        env::set_var("AR", "xtensa-esp32s3-elf-ar");
        env::set_var("CFLAGS", "-mlongcalls -ffunction-sections -fdata-sections");
        env::set_var(
            "CXXFLAGS",
            "-mlongcalls -Wno-frame-address -ffunction-sections -fdata-sections",
        );
    }

    let mut build = cc::Build::new();

    build
        .cpp(true)
        .warnings(false)
        .include(".")
        .include("freetype2/include")
        .define("FT2_BUILD_LIBRARY", None);

    add_sources(
        &mut build,
        "freetype2/src",
        &[
            "autofit/autofit",
            "base/ftbase",
            "base/ftbbox",
            "base/ftbdf",
            "base/ftbitmap",
            "base/ftcid",
            "base/ftdebug",
            "base/ftfstype",
            "base/ftgasp",
            "base/ftglyph",
            "base/ftgxval",
            "base/ftinit",
            "base/ftmm",
            "base/ftotval",
            "base/ftpatent",
            "base/ftpfr",
            "base/ftstroke",
            "base/ftsynth",
            "base/ftsystem",
            "base/fttype1",
            "base/ftwinfnt",
            "bdf/bdf",
            "bzip2/ftbzip2",
            "cache/ftcache",
            "cff/cff",
            "cid/type1cid",
            "gzip/ftgzip",
            "lzw/ftlzw",
            "pcf/pcf",
            "pfr/pfr",
            "psaux/psaux",
            "pshinter/pshinter",
            "psnames/psnames",
            "raster/raster",
            "sdf/sdf",
            "svg/svg",
            "sfnt/sfnt",
            "smooth/smooth",
            "truetype/truetype",
            "type1/type1",
            "type42/type42",
            "winfonts/winfnt",
        ],
    );

    build.compile("freetype2");
}
