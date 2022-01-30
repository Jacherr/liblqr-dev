extern crate bindgen;

fn main() {
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=lqr");
    println!("cargo:rustc-link-lib=glib-2.0");
}
