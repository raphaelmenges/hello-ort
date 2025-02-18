use ort::{
    execution_providers::{ExecutionProvider, XNNPACKExecutionProvider},
    session::Session,
};

const YOLOV8M_URL: &str =
    "https://parcel.pyke.io/v2/cdn/assetdelivery/ortrsv2/ex_models/yolov8m.onnx";

fn main() -> anyhow::Result<()> {
    let mut builder = Session::builder()?;

    let ep = XNNPACKExecutionProvider::default();
    if !ep.is_available()? {
        eprintln!("Please compile ONNX Runtime with XNNPACK!");
        std::process::exit(1);
    }
    ep.register(&mut builder)?;

    let _session = builder.commit_from_url(YOLOV8M_URL)?;
    Ok(())
}
