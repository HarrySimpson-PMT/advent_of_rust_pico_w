use defmt::{info, warn};
use embassy_net::tcp::TcpSocket;
use embassy_net::Stack;
use embassy_time::Duration;
use embedded_io_async::Write;
use heapless::String;

pub struct TcpServer<'a> {
    stack: &'a Stack<'a>,
}

impl<'a> TcpServer<'a> {
    pub fn new(stack: &'a Stack<'a>) -> Self {
        TcpServer { stack }
    }

    pub async fn listen(
        &self,
        port: u16,
        handler: impl Fn(&String<20000>) -> String<100>,
    ) -> Result<(), &'static str> {
        let mut rx_buffer = [0; 4096];
        let mut tx_buffer = [0; 4096];
        let mut socket = TcpSocket::new(*self.stack, &mut rx_buffer, &mut tx_buffer);
    
        socket.set_timeout(Some(Duration::from_secs(10)));
    
        if let Err(e) = socket.accept(port).await {
            warn!("accept error: {:?}", e);
            return Err("Failed to accept connection");
        }
    
        info!("Connection established with {:?}", socket.remote_endpoint());
    
        let mut header_buf = [0; 16];
        let mut accumulated_data = String::<20000>::new();
    
        // Read the header
        match socket.read(&mut header_buf).await {
            Ok(n) => {
                if n == 0 {
                    warn!("Connection closed before receiving header.");
                    return Err("Connection closed early");
                }
    
                let header = core::str::from_utf8(&header_buf[..n]).unwrap_or("");
                let expected_length = match header.strip_prefix("LEN:") {
                    Some(len_str) => len_str.trim().parse::<usize>().unwrap_or(0),
                    None => {
                        warn!("Invalid header format: {:?}", header);
                        return Err("Invalid header format");
                    }
                };
    
                if expected_length > 20000 {
                    warn!("Message length exceeds maximum capacity.");
                    return Err("Message too large");
                }
    
                info!("Expecting message of length: {}", expected_length);
    
                let mut total_received = 0;
                let mut buf = [0; 4096];
                while total_received < expected_length {
                    let n = match socket.read(&mut buf).await {
                        Ok(0) => {
                            warn!("Connection closed before receiving complete message.");
                            return Err("Connection closed early");
                        }
                        Ok(n) => n,
                        Err(e) => {
                            warn!("read error: {:?}", e);
                            return Err("Error reading data");
                        }
                    };
    
                    if accumulated_data
                        .push_str(core::str::from_utf8(&buf[..n]).unwrap_or(""))
                        .is_err()
                    {
                        warn!("Accumulated data exceeds capacity.");
                        return Err("Accumulated data exceeds capacity");
                    }
    
                    total_received += n;
                    info!("Received {}/{} bytes.", total_received, expected_length);
                }    
    
                let response = handler(&accumulated_data);
    
                if let Err(e) = socket.write_all(response.as_bytes()).await {
                    warn!("write error: {:?}", e);
                    return Err("Error writing response");
                }
            }
            Err(_) => return Err("Error reading header"),
        }
    
        Ok(())
    }
    
}
