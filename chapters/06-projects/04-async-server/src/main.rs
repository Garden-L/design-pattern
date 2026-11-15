use async_server::{Job, ServerBuilder};
use tokio::sync::{mpsc, oneshot};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = ServerBuilder::new().workers(2).capacity(1).build()?;
    server.start()?;
    let (sender, input) = mpsc::channel(1);
    let (_cancel, cancelled) = oneshot::channel();
    let producer = async move {
        for job in [Job { id: 7, value: 3 }, Job { id: 2, value: 4 }] {
            sender.send(job).await.expect("server input open");
        }
    };
    let (_, results) = tokio::join!(producer, server.serve(input, cancelled));
    for result in results? {
        println!("{}:{}", result.id, result.value);
    }
    Ok(())
}
