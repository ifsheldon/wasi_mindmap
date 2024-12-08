use std::path::PathBuf;
use std::process::Command;

fn main() {
    let workspace_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned();

    let component_names = [
        "guest-adder-rs",
        "guest-interfaced-adder-rs",
        "guest-largestring-rs",
    ];

    let component_module_names = [
        "guest_adder_rs.wasm",
        "guest_interfaced_adder_rs.wasm",
        "guest_largestring_rs.wasm",
    ];

    // First, build components explicitly
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

    let py_component_module_names = ["guest_adder_py.wasm", "guest_interfaced_adder_py.wasm"];

    for py_component_module_name in py_component_module_names {
        let component_path = workspace_dir
            .join("guest-adder-py")
            .join(py_component_module_name);
        if !component_path.exists() {
            panic!(
                "Required wasip2 module from guest-adder-py not found at: {}",
                component_path.display()
            );
        }
    }

    // Tell cargo to rerun if the sources change
    for component_name in component_names {
        println!(
            "cargo:rerun-if-changed={}",
            workspace_dir.join(component_name).join("src").display()
        );
    }
}
