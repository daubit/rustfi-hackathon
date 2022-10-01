fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Only build once becaue we need to manually fix the files
    //tonic_build::configure()
    //    .out_dir("./src/util/proto")
    //    .compile(
    //        &["./protos/query/query.proto"],
    //        &["./protos/deps", "./protos/query"],
    //    )?;

    Ok(())
}
