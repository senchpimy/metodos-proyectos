extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/contar.c")
        .compile("libdoubler.a");
}
