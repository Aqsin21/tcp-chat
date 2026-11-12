use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use tokio::sync::broadcast;

const ADDRESS: &str = "127.0.0.1:8080";

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind(ADDRESS).await?;
    let (sender, _) = broadcast::channel::<String>(100);
    println!("Chat server listening on {ADDRESS}");

    loop {
        match listener.accept().await {
            Ok((stream, address)) => {
                let receiver = sender.subscribe();
                let sender = sender.clone();
                tokio::spawn(async move {
                    if let Err(error) = handle_connection(stream, sender, receiver).await {
                        eprintln!("connection error for {address}: {error}");
                    }
                });
            }
            Err(error) => eprintln!("failed to accept connection: {error}"),
        }
    }
}

async fn handle_connection<T>(
    stream: T,
    sender: broadcast::Sender<String>,
    mut receiver: broadcast::Receiver<String>,
) -> std::io::Result<()>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let (reader, mut writer) = tokio::io::split(stream);
    let mut lines = BufReader::new(reader).lines();

    loop {
        tokio::select! {
            line = lines.next_line() => {
                match line? {
                    Some(line) => { sender.send(format!("{line}\n")).ok(); }
                    None => break,
                }
            }
            message = receiver.recv() => {
                match message {
                    Ok(message) => writer.write_all(message.as_bytes()).await?,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::handle_connection;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, duplex};
    use tokio::sync::broadcast;

    #[tokio::test]
    async fn broadcasts_messages_to_all_clients_including_sender() {
        let (sender, _) = broadcast::channel(16);
        let (first_client, first_server) = duplex(1024);
        let (second_client, second_server) = duplex(1024);
        let first_receiver = sender.subscribe();
        let second_receiver = sender.subscribe();

        let first_task = tokio::spawn(handle_connection(
            first_server,
            sender.clone(),
            first_receiver,
        ));
        let second_task = tokio::spawn(handle_connection(second_server, sender, second_receiver));

        let (mut first_reader, mut first_writer) = tokio::io::split(first_client);
        let (mut second_reader, mut second_writer) = tokio::io::split(second_client);
        first_writer
            .write_all(b"hello\n")
            .await
            .expect("failed to send message");

        let mut first_response = String::new();
        let mut second_response = String::new();
        BufReader::new(&mut first_reader)
            .read_line(&mut first_response)
            .await
            .expect("failed to read sender response");
        BufReader::new(&mut second_reader)
            .read_line(&mut second_response)
            .await
            .expect("failed to read recipient response");

        assert_eq!(first_response, "hello\n");
        assert_eq!(second_response, "hello\n");

        first_writer
            .shutdown()
            .await
            .expect("failed to disconnect first client");
        second_writer
            .write_all(b"still here\n")
            .await
            .expect("failed to send after disconnect");

        let mut second_follow_up = String::new();
        BufReader::new(&mut second_reader)
            .read_line(&mut second_follow_up)
            .await
            .expect("failed to read after disconnect");
        assert_eq!(second_follow_up, "still here\n");

        first_task.abort();
        second_task.abort();
    }
}
