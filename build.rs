use reqwest::blocking::get;
use std::{
    fs::{File, create_dir_all},
    io::{Cursor, Read, Write},
};
use zip::ZipArchive;

static ORT_ARTIFACT_URL: &str = "https://github.com/alfatraining/ort-artifacts-staging/releases/download/v1.21.0-openvino-windows/ort-rel-1.21.0-x86_64-pc-windows-msvc-md.zip";
static OPENVINO_DISTRIBUTION_URL: &str = "https://storage.openvinotoolkit.org/repositories/openvino/packages/2025.0/windows/openvino_toolkit_windows_2025.0.0.17942.1f68be9f594_x86_64.zip";

fn fetch_and_extract(url: String, files: Vec<(String, String)>) {
    let mut request = get(url).expect("Cannot request archive.");

    let mut buf = Vec::<u8>::new();
    request.read_to_end(&mut buf).expect("Cannot read archive.");

    let reader = Cursor::new(buf);
    let mut zip = ZipArchive::new(reader).expect("Cannot incept unzipper.");

    for (path, name) in files {
        let mut buf = Vec::<u8>::new();
        zip.by_name(format!("{}/{}", path, name).as_str())
            .expect("Cannot find file in zip.")
            .read_to_end(&mut buf)
            .expect("Cannot read file in zip.");

        File::create(format!("./{}", name))
            .expect("Cannot create file.")
            .write_all(&buf)
            .expect("Cannot write file.");

        // Already put .dll's into binary folder.
        if name.ends_with(".dll") {
            let create_next_to_binary = |target| {
                create_dir_all(target).expect("Cannot create target dir.");
                File::create(format!("{target}/{name}"))
                    .expect("Cannot create file.")
                    .write_all(&buf)
                    .expect("Cannot write file.");
            };

            create_next_to_binary("./target/debug");
            create_next_to_binary("./target/release");
        }
    }
}

fn main() {
    // Prepare ONNX runtime.
    fetch_and_extract(
        String::from(ORT_ARTIFACT_URL),
        vec![
            (
                String::from("onnxruntime/lib"),
                String::from("onnxruntime.lib"),
            ),
            (
                String::from("onnxruntime/lib"),
                String::from("onnxruntime_providers_openvino.dll"),
            ),
            (
                String::from("onnxruntime/lib"),
                String::from("onnxruntime_providers_shared.dll"),
            ),
        ],
    );

    // Prepare OpenVINO.
    fetch_and_extract(
        String::from(OPENVINO_DISTRIBUTION_URL),
        vec![
            (
                String::from(
                    "openvino_toolkit_windows_2025.0.0.17942.1f68be9f594_x86_64/runtime/bin/intel64/Release",
                ),
                String::from("openvino.dll"),
            ),
            (
                String::from(
                    "openvino_toolkit_windows_2025.0.0.17942.1f68be9f594_x86_64/runtime/3rdparty/tbb/bin",
                ),
                String::from("tbb12.dll"),
            ),
        ],
    );
}
