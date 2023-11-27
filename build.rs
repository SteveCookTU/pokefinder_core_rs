fn main() -> miette::Result<()> {
    let dst = cmake::build("PokeFinder/Source/Core");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=PokeFinderCore");

    let path = std::path::PathBuf::from("PokeFinder/Source");
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path]).build()?;
    b.flag_if_supported("-std=c++17").compile("pokefinder");

    println!("cargo:rerun-if-changed=src/lib.rs");

    Ok(())
}