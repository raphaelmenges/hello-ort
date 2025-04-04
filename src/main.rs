use ort::session::Session;

const YOLOV8M_URL: &str =
    "https://parcel.pyke.io/v2/cdn/assetdelivery/ortrsv2/ex_models/yolov8m.onnx";

fn main() -> anyhow::Result<()> {
    let builder = Session::builder()?;
    let _session = builder.commit_from_url(YOLOV8M_URL)?;
    println!("{}", "This did not crash, good!");
    Ok(())
}
