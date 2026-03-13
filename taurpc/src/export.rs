use anyhow::{Context, Result, bail};
use heck::ToLowerCamelCase;
use itertools::Itertools;
use specta::TypeCollection;
use specta::datatype::{Function, FunctionReturnType};
use specta_typescript as ts;
use specta_typescript::Typescript;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

static PACKAGE_JSON: &str = r#"
{
    "name": ".taurpc",
    "types": "index.ts"
}
"#;

static BOILERPLATE_TS_IMPORT: &str = r#"

import { createTauRPCProxy as createProxy, type InferCommandOutput } from '@fltsci/taurpc'

type TAURI_CHANNEL<T> = (response: T) => void
"#;

static BOILERPLATE_TS_EXPORT: &str = r#"

const createTauRPCProxy = () => createProxy<Router>(ARGS_MAP)

export {
  type InferCommandOutput,
  createTauRPCProxy
}
"#;

/// Export the generated TS types with the code necessary for generating the client proxy.
///
/// By default, if the `export_to` attribute was not specified on the procedures macro, there will
/// be nothing exported. Otherwise the code will just be export to the .ts file specified by the user.
pub(super) fn export_types(
    export_path: impl AsRef<Path>,
    args_map: BTreeMap<String, String>,
    export_config: ts::Typescript,
    functions: BTreeMap<String, Vec<Function>>,
    type_map: TypeCollection,
) -> Result<()> {
    let path = export_path.as_ref();
    if path.extension() != Some(OsStr::new("ts")) {
        bail!("`export_to` path should be a ts file");
    }

    if let Some(parent) = path.parent() {
        match std::fs::create_dir_all(parent) {
            Ok(_) => (),
            Err(e) => {
                println!("Failed to create directory for exported bindings: {:?}", e);
            }
        }
    }

    // Export all referenced types via specta.
    let types = match export_config
        .export(&type_map)
        .context("Failed to generate types with specta")
    {
        Ok(types) => types,
        Err(e) => {
            println!("Failed to generate types with specta: {:?}", e);
            "".to_string()
        }
    };

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
        .context("Cannot open bindings file")?;

    // Write specta-generated types (includes header + framework prelude + type definitions)
    try_write(&mut file, &types);
    // Append our IPC boilerplate
    try_write(&mut file, BOILERPLATE_TS_IMPORT);

    let args_entries: String = args_map
        .iter()
        .map(|(k, v)| format!("'{k}':'{v}'"))
        .join(", ");
    let router_args = format!("{{ {args_entries} }}");

    try_write(&mut file, &format!("const ARGS_MAP = {router_args}\n"));
    let functions_router = generate_functions_router(functions, type_map, &export_config);
    try_write(&mut file, &functions_router);
    try_write(&mut file, BOILERPLATE_TS_EXPORT);

    if path
        .to_string_lossy()
        .replace("\\", "/")
        .ends_with("node_modules/.taurpc/index.ts")
    {
        let package_json_path = path
            .parent()
            .map(|path| path.join("package.json"))
            .context("Failed to create 'package.json' path")?;

        std::fs::write(package_json_path, PACKAGE_JSON)
            .context("failed to create 'package.json'")?;
    }

    Ok(())
}

fn generate_functions_router(
    functions: BTreeMap<String, Vec<Function>>,
    type_map: TypeCollection,
    export_config: &Typescript,
) -> String {
    let functions = functions
        .iter()
        .map(|(path, path_functions)| {
            let mut function_names_and_funcs: Vec<_> =
                path_functions.iter().map(|f| (f.name(), f)).collect();
            function_names_and_funcs.sort_by(|a, b| a.0.cmp(b.0));

            let functions = function_names_and_funcs
                .iter()
                .filter_map(|(_, function)| {
                    match generate_function(function, export_config, &type_map) {
                        Ok(f) => Some(f),
                        Err(_) => None,
                    }
                })
                .collect::<Vec<_>>()
                .join(", \n");

            format!(r#""{path}": {{{functions}}}"#)
        })
        .collect::<Vec<String>>()
        .join(",\n");

    format!("export type Router = {{ {functions} }};\n")
}

fn generate_function(
    function: &Function,
    export_config: &Typescript,
    type_map: &TypeCollection,
) -> Result<String> {
    let args = function
        .args()
        .iter()
        .map(|(name, typ)| {
            ts::primitives::inline(export_config, type_map, typ)
                .map(|ty| format!("{}: {}", name.to_lower_camel_case(), ty))
                .map_err(|e| anyhow::anyhow!(e))
        })
        .collect::<Result<Vec<_>, _>>()
        .context("An error occured while generating command args")?
        .join(", ");

    let return_ty = match function.result() {
        Some(FunctionReturnType::Value(t)) => {
            ts::primitives::inline(export_config, type_map, t).map_err(|e| anyhow::anyhow!(e))?
        }
        // TODO: handle result types
        Some(FunctionReturnType::Result(t, _e)) => {
            ts::primitives::inline(export_config, type_map, t).map_err(|e| anyhow::anyhow!(e))?
        }
        None => "void".to_string(),
    };

    let name = match function.name().split_once("_taurpc_fn__") {
        Some(thing) => thing.1,
        None => return Err(anyhow::anyhow!("Function name is not valid")),
    };

    Ok(format!(r#"{name}: ({args}) => Promise<{return_ty}>"#))
}

fn try_write(file: &mut File, data: &str) {
    match file.write_all(data.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error writing to file: {e:?}");
        }
    };
}
