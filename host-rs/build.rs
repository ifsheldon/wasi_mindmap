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
        "guest-kv-store-rs",
    ];

    let component_module_names = [
        "guest_adder_rs.wasm",
        "guest_interfaced_adder_rs.wasm",
        "guest_kv_store_rs.wasm",
    ];

    println!("cargo:warning=Running Step 1: Build required components explicitly");
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
            .unwrap_or_else(|_| panic!("Failed to build crate {component_name}"));

        assert!(status.success(), "Failed to build crate {component_name}");
        println!("Finished building {component_name}");
    }

    println!(
        "cargo:warning=Running Step 2: Check if required artifacts (i.e., wasip2 modules) from Rust code exist"
    );
    let target_dir = workspace_dir.join("target").join("wasm32-wasip2");

    for component_module_name in component_module_names {
        let artifact_path = target_dir.join("release").join(component_module_name);
        assert!(
            artifact_path.exists(),
            "Required wasip2 module from guest-adder-rs not found at: {}",
            artifact_path.display()
        );
        // Tell cargo to rerun if artifact changes
        println!("cargo:rerun-if-changed={}", artifact_path.display());
    }

    let py_component_module_names = ["guest_adder_py.wasm", "guest_interfaced_adder_py.wasm"];
    println!(
        "cargo:warning=Running Step 2: Check if required artifacts (i.e., wasip2 modules) from Python code exist"
    );
    for py_component_module_name in py_component_module_names {
        let component_path = workspace_dir
            .join("guest-adder-py")
            .join(py_component_module_name);
        assert!(
            component_path.exists(),
            "Required wasip2 module from guest-adder-py not found at: {}",
            component_path.display()
        );
    }

    // Tell cargo to rerun if the sources change
    for component_name in component_names {
        println!(
            "cargo:rerun-if-changed={}",
            workspace_dir.join(component_name).join("src").display()
        );
    }
    println!("cargo:warning=Finished prebuild checks for required components");
}
