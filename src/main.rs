use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;

#[tokio::main]
async fn main() {

    let address = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&address).await.expect("Cant find address");
    
    let ( tx,rx) = broadcast::channel::<String>(10);

    while let Ok((mut stream,_)) = listener.accept().await {

        let tx = tx.clone();

        let mut rx = rx.resubscribe();

        tokio::spawn(async move{
            let (read, mut write) = stream.split();
            
            let mut reader = BufReader::new(read);
            let mut line = String::new();

            loop {
                // Read data from the stream
                // let bytes_read = match reader.read_line(&mut line).await {
                //     Ok(0) => {
                //         println!("Connection closed");
                //         return;
                //     }
                //     Ok(n) => {
                //         write.write_all(format!("> We heard like {}",line).as_bytes()).await.expect("Failed to write data");
                //         line.clear();
                //     },
                //     Err(e) => {
                //         println!("Error reading from stream: {}", e);
                //         return;
                //     }
                // };
               
               tokio::select! {
                  bytes_read = reader.read_line(&mut line) =>{
                    if bytes_read.unwrap() == 0 {
                        println!("Connection closed");
                        return;
                    }
                    tx.send(line.clone()).unwrap();
                  },
                  msg = rx.recv()=>{
                    write.write_all(format!("> {}", msg.unwrap()).as_bytes()).await.expect("Failed to write data");
                    line.clear();
                  }
               } 
            }
        });
    }
    
       
    
}