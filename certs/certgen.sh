#!/bin/bash

# Generate CA key
openssl genrsa -out ca.key 4096

# Generate CA certificate
openssl req -new -x509 -key ca.key -sha256 -days 365 -out ca.cert -subj "/C=US/ST=PHL/O=Drexel-University"

# Generate the server key
openssl genrsa -out server.key 4096

# Generate the CSR (Certificate Signing Request)
openssl req -new -key server.key -out server.csr -config certificate.conf

# Generate the server certificate signed by the CA
openssl x509 -req -in server.csr -CA ca.cert -CAkey ca.key -CAcreateserial -out server.crt -days 365 -sha256 -extfile certificate.conf -extensions req_ext
