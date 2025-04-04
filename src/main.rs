use ort::{execution_providers::OpenVINOExecutionProvider, session::Session};

const YOLOV8M_URL: &str =
    "https://parcel.pyke.io/v2/cdn/assetdelivery/ortrsv2/ex_models/yolov8m.onnx";

fn main() -> anyhow::Result<()> {
    let _builder = Session::builder()?
        .with_execution_providers([OpenVINOExecutionProvider::default().build()])?
        .commit_from_url(YOLOV8M_URL)?;
    println!("{}", "This did not crash, good!");
    Ok(())
}
