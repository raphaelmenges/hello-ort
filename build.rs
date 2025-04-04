use reqwest::blocking::get;
use std::{
    fs::{File, exists, remove_file},
    io::{Cursor, Read, Write},
};
use zip::ZipArchive;

static ORT_ARTIFACT_URL: &str = "https://github.com/alfatraining/ort-artifacts-staging/releases/download/v1.21.0-openvino-windows/ort-rel-1.21.0-x86_64-pc-windows-msvc-md.zip";

fn main() {
    #[cfg(any(target_os = "macos", target_os = "ios", target_os = "tvos"))]
    println!("cargo:rustc-link-arg=-fapple-link-rtlib");

    // Request archive.
    let mut request = get(ORT_ARTIFACT_URL).expect("Cannot request precompiled onnxruntime.");
    let mut buf = Vec::<u8>::new();
    request
        .read_to_end(&mut buf)
        .expect("Cannot read precompiled onnxruntime.");

    // Prepare extraction.
    let reader = Cursor::new(buf);
    let mut zip = ZipArchive::new(reader).expect("Cannot incept unzipper.");

    // Extract precompiled library.
    {
        let mut extract = |filename| {
            let mut buf = Vec::<u8>::new();
            zip.by_name(format!("onnxruntime/lib/{}", filename).as_str())
                .expect("Cannot find file in zip.")
                .read_to_end(&mut buf)
                .expect("Cannot read file in zip.");

            let out_path = format!("./{}", filename);
            if exists(&out_path).expect("Cannot check file on disk.") {
                remove_file(&out_path).expect("Cannot delete file on disk.");
            }
            File::create(&out_path)
                .expect("Cannot create file.")
                .write_all(&buf)
                .expect("Cannot fill file.");
        };

        extract("onnxruntime.lib");
        extract("onnxruntime_providers_openvino.dll");
        extract("onnxruntime_providers_shared.dll");
    }
}
