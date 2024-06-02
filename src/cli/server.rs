use color_eyre::eyre::Result;
use s2n_quic::Server;
use std::{path::Path, fs::File, io::{Write, Read}};
use std::net::ToSocketAddrs;
use md5;
use serde_json;
use crate::cli::pdu::PDU;

#[derive(Debug)]
struct ServerOptions {
    address: String,
    port: u16,
    cert: String,
    key: String,
}

#[tokio::main]
async fn run(options: ServerOptions) -> Result<()> {
    let host_port_string = format!("{}:{}", options.address, options.port)
        .to_socket_addrs()?
        .next()
        .unwrap();
    let mut server = Server::builder()
        .with_tls((Path::new(&options.cert), Path::new(&options.key)))?
        .with_io(host_port_string)?
        .start()?;
    println!("{:#?} In server...", options);
    while let Some(mut connection) = server.accept().await {
        tokio::spawn(async move {
            while let Ok(Some(mut stream)) = connection.accept_bidirectional_stream().await {
                tokio::spawn(async move {
                    let mut pdu_bytes = Vec::new();
                    let mut file_content = Vec::new();
                    let mut separator_found = false;

                    while let Ok(Some(data)) = stream.receive().await {
                        if !separator_found {
                            pdu_bytes.extend_from_slice(&data);
                            if let Some(pos) = pdu_bytes.iter().position(|&x| x == b'\n') {
                                separator_found = true;
                                file_content.extend_from_slice(&pdu_bytes[pos+1..]);
                                pdu_bytes.truncate(pos);
                                break;
                            }
                        } else {
                            file_content.extend_from_slice(&data);
                        }
                    }

                    if !separator_found {
                        println!("Separator not found in the received data");
                        return;
                    }

                    let pdu: PDU = serde_json::from_slice(&pdu_bytes).expect("Failed to deserialize PDU");
                    println!("Received PDU: {:?}", pdu);

                    // Receive the rest of the file content
                    while let Ok(Some(data)) = stream.receive().await {
                        file_content.extend_from_slice(&data);
                    }

                    // Verify checksum
                    let checksum_calculated = md5::compute(&file_content);
                    let checksum_calculated_str = format!("{:x}", checksum_calculated);

                    if pdu.checksum == checksum_calculated_str {
                        println!("Checksum verification passed.");
                    } else {
                        println!("Checksum verification failed.");
                    }

                    // Save file to disk
                    let mut file = File::create(&pdu.filename).expect("Unable to create file");
                    file.write_all(&file_content).expect("Unable to write file");
                    println!("File received and saved: {}", pdu.filename);

                    // Send the file back to the client
                    let mut file = File::open(&pdu.filename).expect("Unable to open file");
                    let mut file_content = Vec::new();
                    file.read_to_end(&mut file_content).expect("Unable to read file");
                    stream.send(file_content.into()).await.expect("stream should be open");
                    println!("File sent back to client: {}", pdu.filename);

                    stream.finish().expect("Failed to close stream"); // Explicitly close the stream
                });
            }
        });
    }
    Ok(())
}

pub fn do_server(address: String, port: u16, cert: String, key: String) -> Result<()> {
    println!("Starting server...");
    println!("Listening on {address} using port {port}...");

    let options = ServerOptions {
        address,
        port,
        cert,
        key,
    };

    run(options)?;

    Ok(())
}
