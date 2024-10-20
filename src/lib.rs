use std::{
    error::Error,
    io::{self, Read, Write},
    net::{Shutdown, TcpStream, ToSocketAddrs},
    str::from_utf8,
    time::Duration,
};

pub use cli::Cli;
mod cli;

const MESSAGE_SIZE: usize = 1024;

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let addr = &format!("{}:{}", cli.host, cli.port)
        .to_socket_addrs()?
        .next()
        .ok_or("Invalid address")?;
    let timeout = Duration::from_secs(cli.timeout);

    // Пытаемся подключиться к серверу с таймаутом
    let mut stream = match TcpStream::connect_timeout(&addr, timeout) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Failed to connect to server: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("Successfully connected to server on port {}", cli.port);

    // Устанавливаем таймауты на чтение и запись
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;

    let mut data = [0 as u8; MESSAGE_SIZE];

    loop {
        // Чтение данных из stdin
        data.fill(0);
        let read_bytes = match io::stdin().read(&mut data) {
            Ok(0) => {
                // Если прочли 0 байт, это Ctrl+D, завершаем соединение
                println!("Closing connection (Ctrl+D)");
                stream.shutdown(Shutdown::Both)?;
                break;
            }
            Ok(n) => n,
            Err(e) => {
                println!("Error reading from stdin: {}", e);
                return Err(Box::new(e));
            }
        };

        // Отправляем данные на сервер
        if let Err(e) = stream.write_all(&data[..read_bytes]) {
            println!("Failed to send data to server: {}", e);
            return Err(Box::new(e));
        }

        // Чтение ответа от сервера
        data.fill(0);
        match stream.read(&mut data) {
            Ok(0) => {
                // Если сервер закрыл соединение
                println!("Connection closed by server.");
                break;
            }
            Ok(n) => {
                let text = from_utf8(&data[..n]).unwrap_or("[Invalid UTF-8]");
                print!("{text}");
            }
            Err(e) => {
                println!("Failed to read from server: {}", e);
                return Err(Box::new(e));
            }
        }
    }

    Ok(())
}
