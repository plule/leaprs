use std::{env, path::PathBuf};

use copy_to_output::copy_to_output;

fn main() {
    // Find Leap SDK
    println!(r"cargo:rerun-if-env-changed=LEAPSDK_LIB_PATH");

    let leapsdk_path = env::var("LEAPSDK_LIB_PATH")
        .unwrap_or_else(|_| r"C:\Program Files\Ultraleap\LeapSDK\lib\x64".to_string());

    let leapsdk_path = PathBuf::from(leapsdk_path);

    assert!(
        leapsdk_path.is_dir(),
        "Could not find LeapSDK at the location {}. Install it from https://developer.leapmotion.com/tracking-software-download or set its location with the environment variable LEAPSDK_LIB_PATH.",
        leapsdk_path.display()
    );

    let path_str = leapsdk_path
        .to_str()
        .unwrap_or_else(|| panic!("{} is not a valid path.", leapsdk_path.display()));

    // Link to LeapC.lib
    println!(r"cargo:rustc-link-search={}", path_str);
    println!(r"cargo:rustc-link-lib=static=LeapC");

    // Copy LeapC.dll to the output
    let mut leapcdll_path = leapsdk_path.clone();
    leapcdll_path.push("LeapC.dll");
    let leapcdll_path = leapcdll_path.to_str().unwrap();
    println!("cargo:rerun-if-changed={}", leapcdll_path);
    copy_to_output(leapcdll_path, &env::var("PROFILE").unwrap())
        .expect("Failed to copy LeapC.dll to the output path");
}
