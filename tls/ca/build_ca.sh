#!/bin/bash

# Create directory structure
mkdir -p ca/certs ca/crl ca/newcerts ca/private
chmod 700 ca/private
touch ca/index.txt
echo 1000 > ca/serial

# Generate the ca key
openssl genrsa -aes256 -out ca/private/ca.key.pem 4096
chmod 400 ca/private/ca.key.pem

# Generate the ca certificate
openssl req -config openssl_ca.cnf \
      -subj "/C=DE/L=Germany/O=redhac/CN=redhac-ca" \
      -key ca/private/ca.key.pem \
      -new -x509 -days 7300 -sha256 -extensions v3_ca \
      -out ca/certs/ca.cert.pem
chmod 444 ca/certs/ca.cert.pem

# Verify the certificate
openssl x509 -noout -text -in ca/certs/ca.cert.pem
