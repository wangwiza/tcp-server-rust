use std::error::Error;
use std::sync::mpsc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;

use crate::task::Task;
use crate::task::TaskType;

pub trait ServerTrait {
    fn start_server(
        &self,
        address: String,
        tx: mpsc::Sender<Result<(), Box<dyn Error + Send>>>,
    );
}

pub struct Server;

impl ServerTrait for Server {
    fn start_server(
        &self,
        address: String,
        tx: mpsc::Sender<Result<(), Box<dyn Error + Send>>>,
    ) {
        println!("Starting the server");
        // By default, it will start a worker thread for each CPU core available on the system.
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let listener = TcpListener::bind(&address).await;
            match listener {
                Ok(_) => {
                    tx.send(Ok(())).unwrap();
                }
                Err(e) => {
                    tx.send(Err(Box::new(e))).unwrap();
                    return;
                }
            }
            let listener = listener.unwrap();
            loop {
                match listener.accept().await {
                    Ok((stream, _)) => {
                        tokio::spawn(Self::handle_connection(stream));
                    }
                    Err(e) => {
                        eprintln!("Error accepting connection: {}", e);
                    }
                }
            }
        });
    }
}

impl Server {
    async fn handle_connection(mut stream: TcpStream) {
        let (read_half, mut write_half) = stream.split();
        let mut reader = BufReader::new(read_half);
        let mut line = String::new();

        loop {
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    let response = Self::get_task_value(line.clone()).await;
                    if let Some(r) = response {
                        let _ = write_half.write_all(&[r]).await;
                    }
                    line.clear();
                }
                Err(e) => {
                    eprintln!("Unable to get command due to: {}", e);
                    break;
                }
            }
        }
    }

    async fn get_task_value(buf: String) -> Option<u8> {
        let numbers: Vec<&str> = buf.trim().split(':').collect();
        let task_type_num = numbers.first()?.parse::<u8>().ok()?;
        let seed = numbers.last()?.parse::<u64>().ok()?;
        let task_type = TaskType::from_u8(task_type_num)?;

        let result = match task_type {
            TaskType::CpuIntensiveTask => {
                tokio::task::spawn_blocking(move || Task::execute(task_type_num, seed))
                    .await
                    .ok()?
            }
            TaskType::IOIntensiveTask => Task::execute_async(task_type_num, seed).await,
        };

        Some(result)
    }
}
