use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{tcp::ReadHalf, TcpListener}};

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("localhost:8080").await.unwrap();
    let (mut socket, _addr) = listener.accept().await.unwrap();
    let (read, mut writer) = socket.split();

    let mut reader: BufReader<ReadHalf> = BufReader::new(read);
    let mut line: String = String::new();

    loop {
        let bytes_read: usize = reader.read_line(&mut line).await.unwrap();
        if bytes_read == 0 {
            break;
        }

        writer.write_all(&line.as_bytes()).await.unwrap();
        line.clear();
    }
}
