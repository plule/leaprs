use std::{env, path::PathBuf};

#[cfg(windows)]
const DEFAULT_LEAPSDK_LIB_PATH: &str = r"C:\Program Files\Ultraleap\LeapSDK\lib\x64";

#[cfg(not(windows))]
const DEFAULT_LEAPSDK_LIB_PATH: &str = r"/usr/share/doc/ultraleap-hand-tracking-service";

fn main() {
    // Find Leap SDK
    println!(r"cargo:rerun-if-env-changed=LEAPSDK_LIB_PATH");

    let leapsdk_path =
        env::var("LEAPSDK_LIB_PATH").unwrap_or_else(|_| DEFAULT_LEAPSDK_LIB_PATH.to_string());

    let leapsdk_path = PathBuf::from(leapsdk_path)
        .canonicalize()
        .expect("Failed to canonicalize sdk path");

    if !leapsdk_path.is_dir() {
        println!("cargo:warning=Could not find LeapSDK at the location {}. Install it from https://developer.leapmotion.com/tracking-software-download or set its location with the environment variable LEAPSDK_LIB_PATH.", leapsdk_path.display());
    } else {
        let path_str = leapsdk_path
            .to_str()
            .unwrap_or_else(|| panic!("{} is not a valid path.", leapsdk_path.display()));

        // Link to LeapC.lib
        println!(r"cargo:rustc-link-lib=LeapC");
        println!(r"cargo:rustc-link-search={}", path_str);
    }
}
