fn main() {
    println!(r"cargo:rustc-link-search=C:\Program Files\Ultraleap\LeapSDK\lib\x64");
    println!(r"cargo:rustc-link-lib=static=LeapC");
}
