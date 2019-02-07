use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    for exception in ["SysTick", "PendSV", "SVCall"].into_iter() {
        fs::copy(
            format!("bin/{}_handler.a", exception),
            out_dir.join(format!("libcortex_m_switch_{}_handler.a", exception)),
        )
        .unwrap();
    }
    fs::copy("bin/svc.a", out_dir.join("libsvc.a")).unwrap();
    fs::copy("bin/svc_extra.a", out_dir.join("libsvc_extra.a")).unwrap();

    println!("cargo:rustc-link-search={}", env::var("OUT_DIR").unwrap());
}
