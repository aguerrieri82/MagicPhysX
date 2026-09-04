use std::{env, error::Error, fs, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let generated = PathBuf::from(env::var("DEP_PHYSX_BINDINGS_DIR")?);
    let physx_sys = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../physx-rs/physx-sys");

    let rust_output = PathBuf::from("./src/physx_ffi.rs");
    let csharp_output = PathBuf::from("../MagicPhysX/NativeMethods.g.cs");

    csbindgen::Builder::new()
        .input_bindgen_file(physx_sys.join("src/lib.rs"))
        .input_bindgen_file(generated.join("bindings/physx_generated.rs"))
        .input_bindgen_file(generated.join("structgen_out.rs"))
        .method_filter(|x| !(x == "create_contact_callback" || x == "destroy_contact_callback"))
        .rust_file_header("use super::physx_sys::*;")
        .rust_method_prefix("physx_")
        .csharp_entry_point_prefix("")
        .csharp_namespace("PhysX")
        .csharp_class_name("NativeMethods")
        .csharp_dll_name("physxnative")
        .csharp_class_accessibility("public")
        .generate_to_file(
            &rust_output,
            &csharp_output,
        )?;

    // csbindgen 1.7 emits the pre-Rust-2024 spelling. Keep generated output
    // compatible with current Rust toolchains without hand-editing the file.
    let rust_bindings = fs::read_to_string(&rust_output)?;
    let rust_bindings = rust_bindings.replace("#[no_mangle]", "#[unsafe(no_mangle)]");
    fs::write(rust_output, rust_bindings)?;

    // The public API is generated on the Windows host, but four opaque/layout
    // types have smaller Android arm64 representations. Emit both layouts from
    // the generator so managed Android callers use the same ABI as the NDK
    // wrapper without maintaining a second generated C# file.
    let csharp_bindings = fs::read_to_string(&csharp_output)?;
    let nl = if csharp_bindings.contains("\r\n") { "\r\n" } else { "\n" };
    let replacements = [
        (
            format!("public unsafe partial struct PxSIMDGuard{nl}    {{{nl}        public fixed byte structgen_pad0[8];{nl}    }}"),
            format!("public unsafe partial struct PxSIMDGuard{nl}    {{{nl}#if ANDROID{nl}        public fixed byte structgen_pad0[1];{nl}#else{nl}        public fixed byte structgen_pad0[8];{nl}#endif{nl}    }}"),
        ),
        (
            format!("        public void* userData;{nl}        public fixed byte structgen_pad4[8];{nl}        public float halfHeight;{nl}        public float halfSideExtent;{nl}        public float halfForwardExtent;{nl}        public fixed byte structgen_pad5[4];"),
            format!("        public void* userData;{nl}#if ANDROID{nl}        public fixed byte structgen_pad4[4];{nl}#else{nl}        public fixed byte structgen_pad4[8];{nl}#endif{nl}        public float halfHeight;{nl}        public float halfSideExtent;{nl}        public float halfForwardExtent;{nl}#if !ANDROID{nl}        public fixed byte structgen_pad5[4];{nl}#endif"),
        ),
        (
            format!("        public void* userData;{nl}        public fixed byte structgen_pad4[8];{nl}        public float radius;{nl}        public float height;{nl}        public PxCapsuleClimbingMode climbingMode;{nl}        public fixed byte structgen_pad5[4];"),
            format!("        public void* userData;{nl}#if ANDROID{nl}        public fixed byte structgen_pad4[4];{nl}#else{nl}        public fixed byte structgen_pad4[8];{nl}#endif{nl}        public float radius;{nl}        public float height;{nl}        public PxCapsuleClimbingMode climbingMode;{nl}#if !ANDROID{nl}        public fixed byte structgen_pad5[4];{nl}#endif"),
        ),
        (
            format!("public unsafe partial struct PxTriangleMeshPoissonSampler{nl}    {{{nl}        public fixed byte structgen_pad0[24];{nl}    }}"),
            format!("public unsafe partial struct PxTriangleMeshPoissonSampler{nl}    {{{nl}#if ANDROID{nl}        public fixed byte structgen_pad0[8];{nl}#else{nl}        public fixed byte structgen_pad0[24];{nl}#endif{nl}    }}"),
        ),
    ];
    let mut csharp_bindings = csharp_bindings;
    for (original, replacement) in replacements {
        if !csharp_bindings.contains(&original) {
            return Err(format!("expected generated C# layout was not found: {original}").into());
        }
        csharp_bindings = csharp_bindings.replace(&original, &replacement);
    }
    fs::write(csharp_output, csharp_bindings)?;

    Ok(())
}
