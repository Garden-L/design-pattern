fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pipeline = event_pipeline::EventPipeline::new();
    pipeline.subscribe(|event| println!("{}", event.0));
    for request in ["add 5", "add -2"] {
        pipeline.process(request)?;
    }
    let summary = pipeline.shutdown();
    println!(
        "processed={}, balance={}",
        summary.processed, summary.balance
    );
    Ok(())
}
