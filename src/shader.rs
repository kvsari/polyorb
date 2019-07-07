//! Shader handling stuff
use std::{fs, path};

use shaderc::{ShaderKind, Error, Compiler};

pub fn load(name: &str, entry: &str, kind: ShaderKind) -> Result<Vec<u8>, Error> {
    let mut compiler = Compiler::new()
        .ok_or(Error::NullResultObject("Can't create compiler.".to_owned()))?;

    let filepath = path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("shaders")
        .join(name);

    let contents = fs::read_to_string(&filepath)
        .map_err(|e| Error::NullResultObject(format!("{}", &e)))?;

    let artifact = compiler.compile_into_spirv(&contents, kind, name, entry, None)?;
    
    Ok(artifact.as_binary_u8().to_owned())
}

pub fn load_vert(name: &str, entry: &str) -> Result<Vec<u8>, Error> {
    load(name, entry, ShaderKind::Vertex)
}

pub fn load_frag(name: &str, entry: &str) -> Result<Vec<u8>, Error> {
    load(name, entry, ShaderKind::Fragment)
}
