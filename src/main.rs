use ndarray::{Array, ArrayBase, Dim, OwnedRepr};
use ort::{
    execution_providers::{ExecutionProvider, OpenVINOExecutionProvider}, inputs, session::Session, value::TensorRef,
};

const YOLOV8M_URL: &str = "https://cdn.pyke.io/0/pyke:ort-rs/example-models@0.0.0/yolov8m.onnx";

fn main() -> anyhow::Result<()> {

    let mut builder = Session::builder()?;
    let ep = OpenVINOExecutionProvider::default();
    println!("OpenVINO ep available: {}", ep.is_available()?);
    ep.register(&mut builder)?;
    println!("OpenVINO ep registered!");
    let mut session = builder.commit_from_url(YOLOV8M_URL)?;
    println!("Session built!");
    let input: ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>> = Array::zeros((1, 3, 640, 640));
    let _outputs = session.run(inputs!["images" => TensorRef::from_array_view(&input)?])?;
    println!("{}", "This did not crash, good!");
    Ok(())
}
