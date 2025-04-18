use std::error::Error;
use std::sync::mpsc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Builder;
use std::sync::Arc;

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

/// Use 6 threads for async tasks
const ASYNC_THREADS: usize = 6;
// The other threads can handle the CPU tasks
const RAYON_THREADS: usize = 10;
const CPU_TASK_PROB: f64 = 0.5;

impl ServerTrait for Server {
    fn start_server(
        &self,
        address: String,
        tx: mpsc::Sender<Result<(), Box<dyn Error + Send>>>,
    ) {
        println!("Starting the server");
        // Create a runtime with specified number of worker threads
        let rt = Builder::new_multi_thread()
            .worker_threads(ASYNC_THREADS)
            .enable_all()
            .build()
            .unwrap();

        // Uh... make rayon pool for CPU tasks?
        let rayon_pool = Arc::new(
            rayon::ThreadPoolBuilder::new()
            .num_threads(RAYON_THREADS)
            .build()
            .unwrap()
        );


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
                        tokio::spawn(Self::handle_connection(stream, rayon_pool.clone()));
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
    async fn handle_connection(mut stream: TcpStream, rayon_pool: Arc<rayon::ThreadPool>) {
        let (read_half, mut write_half) = stream.split();
        let mut reader = BufReader::new(read_half);
        let mut line = String::new();

        loop {
            match reader.read_line(&mut line).await {
                Ok(0) => break,
                Ok(_) => {
                    let response = Self::get_task_value(line.clone(), rayon_pool.clone()).await;
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

    async fn get_task_value(buf: String, rayon_pool: Arc<rayon::ThreadPool>) -> Option<u8> {
        let numbers: Vec<&str> = buf.trim().split(':').collect();
        let task_type_num = numbers.first()?.parse::<u8>().ok()?;
        let seed = numbers.last()?.parse::<u64>().ok()?;
        let task_type = TaskType::from_u8(task_type_num)?;

        let result = match task_type {
            TaskType::CpuIntensiveTask => {
                let (tx, rx) = tokio::sync::oneshot::channel();
                rayon_pool.spawn(move || {
                    let result = Task::execute(task_type_num, seed);
                    let _ = tx.send(result);
                });
                rx.await.ok()?
            }
            TaskType::IOIntensiveTask => Task::execute_async(task_type_num, seed).await,
            TaskType::MysteryTask => {
                let run_as_cpu_task = rand::random_bool(CPU_TASK_PROB);
                if run_as_cpu_task {
                    let (tx, rx) = tokio::sync::oneshot::channel();
                    rayon_pool.spawn(move || {
                        let result = Task::execute(task_type_num, seed);
                        let _ = tx.send(result);
                    });
                    rx.await.ok()?
                } else {
                    Task::execute_async(task_type_num, seed).await
                }
            }
        };

        Some(result)
    }
}
