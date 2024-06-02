# QUIC File Transfer Protocol

This project implements a file transfer protocol over QUIC, supporting the transfer of text and binary files, such as images. It ensures secure communication using TLS certificates and includes a feature for verifying file integrity through checksums.

## Features

- Transfer of text and binary files (e.g., images) over QUIC.
- Secure communication using TLS certificates for encryption.
- File integrity verification using checksums.
- Handles both client and server operations.
- Supports both upload and download of files.

## Requirements

- **Rust**: Ensure Rust is installed on your system.
- **OpenSSL**: If you wish to generate your own keys and certificates.

### Generating Your Own Certificates

The `certs` folder already contains keys and certificates for both client and server generated by me. If you wish to create your own, follow the instructions below.

1. Ensure **OpenSSL** is installed on your system.

2. Run the `certgen.sh` shell script:
    ```sh
    # Run the certificate generation script
    ./certgen.sh
    ```

3. Make changes in the `certificate.conf` file if needed.

## Usage

1. Build the project (optional, as running the cargo run command directly will also build the project before executing):
    ```sh
    cargo build
    ```

2. Start the server:
    ```sh
    cargo run -- server --cert ./certs/server.crt --key ./certs/server.key
    ```

3. Run the client:
    ```sh
    cargo run --bin quicrs -- client --address 127.0.0.1 --port 54321 --cert ./certs/ca.cert
    ```

Make sure to run these commands in the root folder where the `Cargo.toml` file is located.

The client will prompt for a filename. The file should be present in the root folder.

## How It Works

1. **Client Initialization**: The client is started and connects to the server using the provided address and port. It then establishes a secure connection using TLS certificates.
2. **File Selection**: The user is prompted to enter the filename to upload. The file should be present in the root folder.
3. **PDU Creation**: The client creates a Protocol Data Unit (PDU) that includes the filename and checksum of the file.
4. **File Transfer**: The client sends the PDU and file content to the server. The server receives the PDU and file content, verifies the checksum, and saves the file to disk.
5. **File Download**: The server sends the file back to the client. The client receives the file and saves it as `received_<filename>`.

## Extra Credits

- Uploaded source code on GitHub
- Demo of the FTP prototype: [https://youtu.be/Q8x3nu8e3vk](https://youtu.be/Q8x3nu8e3vk)
- Used asynchronous tasks to handle multiple clients by creating a new task for each accepted connection.
- Included a learning summary
- Used systems programming language (Rust)
