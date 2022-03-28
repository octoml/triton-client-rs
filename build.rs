/// This file generates the client libraries from Triton's shared proto-buf definitions.
use anyhow::{Context, Result};
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Return a list of all proto files in the given directory, recursively.
fn get_protobuf_paths<P: AsRef<Path>>(directory: P) -> std::io::Result<Vec<PathBuf>> {
    let mut paths: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(directory) {
        let path = entry?.into_path();
        if path.extension() == Some(OsStr::new("proto")) {
            paths.push(path.to_path_buf());
        }
    }
    Ok(paths)
}

fn main() -> Result<()> {
    // The toplevel types directory.
    let pb_dir: std::path::PathBuf = env::var("TRITON_PROTOBUF")
        .ok()
        .unwrap_or(concat!(env!("CARGO_MANIFEST_DIR"), "/common/protobuf").to_string()).into();

    println!("cargo:rerun-if-changed={}", &pb_dir.display());

    let protobuf_paths = get_protobuf_paths(&pb_dir).context(format!(
        "failed to fetch protobuf paths for {}",
        pb_dir.display()
    ))?;

    for path in &protobuf_paths {
        println!("cargo:rerun-if-changed={}", path.display());
    }

    tonic_build::configure()
        // // Ensure that all protobufs have Serde traits defined, so we can JSON-serialize them
        // // when needed, e.g., for logging.
        // .type_attribute(".", "#[derive(::serde::Serialize, ::serde::Deserialize)]")
        // // We need to provide a Serde implementation for the protobuf Timestamp type.
        // .field_attribute("create_time", "#[serde(with = \"crate::serde_timestamp\")]")
        // .field_attribute("update_time", "#[serde(with = \"crate::serde_timestamp\")]")
        // .field_attribute("expiry_time", "#[serde(with = \"crate::serde_timestamp\")]")
        // .field_attribute("timestamp", "#[serde(with = \"crate::serde_timestamp\")]")
        // .field_attribute("start_time", "#[serde(with = \"crate::serde_timestamp\")]")
        // .field_attribute("end_time", "#[serde(with = \"crate::serde_timestamp\")]")
        // .field_attribute(
        //     "dispatch_time",
        //     "#[serde(with = \"crate::serde_timestamp\")]",
        // )
        // .field_attribute(
        //     "last_activity_time",
        //     "#[serde(with = \"crate::serde_timestamp\")]",
        // )
        // // We need to provide a Serde implementation for the protobuf Duration type.
        // .field_attribute(
        //     "MeasurementSpec.warmup",
        //     "#[serde(with = \"crate::serde_duration\")]",
        // )
        // .field_attribute(
        //     "MeasurementSpec.cooldown",
        //     "#[serde(with = \"crate::serde_duration\")]",
        // )
        // .field_attribute(
        //     "MeasurementRequest.timeout",
        //     "#[serde(with = \"crate::serde_duration\")]",
        // )
        // .field_attribute(
        //     "MeasurementResponse.duration",
        //     "#[serde(with = \"crate::serde_duration\")]",
        // )
        // .field_attribute(
        //     "MeasurementResult.runtimes",
        //     "#[serde(with = \"crate::serde_duration_vec\")]",
        // )
        // .field_attribute(
        //     "OpMeasurement.runtime",
        //     "#[serde(with = \"crate::serde_duration\")]",
        // )
        // // We need to provide a Serde implementation for the protobuf FieldMask type.
        // .field_attribute("update_mask", "#[serde(with = \"crate::serde_fieldmask\")]")
        // .field_attribute("select_mask", "#[serde(with = \"crate::serde_fieldmask\")]")
        .build_server(true)
        .compile(&protobuf_paths, &[&pb_dir])
        .context("Unable to compile protobufs for Triton client")?;

    Ok(())
}
