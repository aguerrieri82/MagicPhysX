use std::{
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    csbindgen::Builder::new()
        .input_bindgen_file("src/physx/lib.rs")
        .input_bindgen_file("src/physx/physx_generated.rs")
        .input_bindgen_file("src/physx/x86_64-pc-windows-msvc/structgen.rs")
        .method_filter(|x| !(x == "create_contact_callback" || x == "destroy_contact_callback"))
        .rust_file_header("use super::physx_sys::*;")
        .rust_method_prefix("physx_")
        .csharp_entry_point_prefix("")
        .csharp_namespace("PhysX")
        .csharp_class_name("NativeMethods")
        .csharp_dll_name("physxnative")
        .csharp_class_accessibility("public")
        .generate_to_file(
            "./src/physx_ffi.rs",
            "../MagicPhysX/NativeMethods.g.cs",
        )?;

    Ok(())
}