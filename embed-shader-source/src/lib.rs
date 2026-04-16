use std::{fs::File, io::Read, path::PathBuf};

use proc_macro::TokenStream;
use quote::quote;
use syn::{LitByteStr, LitStr, parse_macro_input};

fn obfuscate(data: &mut [u8]) {
    for byte in data {
        *byte = byte.wrapping_add(128);
    }
}

#[proc_macro]
pub fn embed_shader_source(input: TokenStream) -> TokenStream {
    let filename = parse_macro_input!(input as LitStr);

    let mut full_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    full_path.push("src/engine/graphics/shaders");
    full_path.push(filename.value());

    let mut file = File::open(&full_path).unwrap();
    let mut source = Vec::new();
    file.read_to_end(&mut source).unwrap();
    let len = source.len();

    obfuscate(&mut source);

    let path = LitStr::new(full_path.to_str().unwrap(), filename.span());

    quote! {
        {
            static mut DATA: [u8; #len] = [#(#source), *];

            // Trigger recompilation when file changes.
            let _ = include_bytes!(#path);

            unsafe {
                for byte in &mut DATA {
                    *byte = byte.wrapping_add(128);
                }

                let source = str::from_utf8(&DATA).unwrap();
                let filename = #filename.to_string();

                crate::engine::graphics::ShaderSource { source, filename }
            }
        }
    }.into()
}