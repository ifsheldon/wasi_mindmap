use std::path::PathBuf;
use std::process::Command;

fn main() {
    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned();

    // First, build `guest-adder-rs` and `guest-largestring-rs` components explicitly
    let component_names = ["guest-adder-rs", "guest-largestring-rs"];
    for component_name in component_names {
        let status = Command::new("cargo")
            .current_dir(&workspace_dir)
            .args([
                "build",
                "-p",
                component_name,
                "--release",
                "--target",
                "wasm32-wasip2",
            ])
            .status()
            .expect(format!("Failed to build crate {}", component_name).as_str());

        if !status.success() {
            panic!("Failed to build crate {}", component_name);
        }

        println!("Finished building {}", component_name);
    }

    // Check if the artifacts (i.e., wasip2 modules) exist
    let target_dir = workspace_dir.join("target").join("wasm32-wasip2");
    let component_module_names = ["guest_adder_rs.wasm", "guest_largestring_rs.wasm"];
    for component_module_name in component_module_names {
        let artifact_path = target_dir.join("release").join(component_module_name);
        if !artifact_path.exists() {
            panic!(
                "Required wasip2 module from guest-adder-rs not found at: {}",
                artifact_path.display()
            );
        }
        // Tell cargo to rerun if artifact changes
        println!("cargo:rerun-if-changed={}", artifact_path.display());
    }

    let py_adder_guest_path = workspace_dir
        .join("guest-adder-py")
        .join("guest_adder_py.wasm");
    if !py_adder_guest_path.exists() {
        panic!(
            "Required wasip2 module from guest-adder-py not found at: {}",
            py_adder_guest_path.display()
        );
    }

    // Tell cargo to rerun if the sources change
    for component_name in component_names {
        println!(
            "cargo:rerun-if-changed={}",
            workspace_dir.join(component_name).join("src").display()
        );
    }
}
