#!/bin/bash

# Create directory structure
mkdir -p intermediate/certs intermediate/crl intermediate/csr intermediate/newcerts intermediate/private
chmod 700 intermediate/private
touch intermediate/index.txt
echo 1000 > intermediate/serial
echo 1000 > intermediate/crlnumber

# Generate the ca key
openssl genrsa -aes256 -out intermediate/private/intermediate.key.pem 4096
chmod 400 intermediate/private/intermediate.key.pem

# Generate the CSR
#openssl req -config openssl_intermediate.cnf -new -sha256 \
openssl req -config openssl.cnf -new -sha256 \
      -subj "/C=DE/L=Germany/O=redhac/CN=redhac-it-ca" \
      -key intermediate/private/intermediate.key.pem \
      -out intermediate/csr/intermediate.csr.pem
#      -subj "/CN=redhac-it-ca"

# Generate the ca certificate
openssl ca -config openssl_ca.cnf -extensions v3_intermediate_ca \
      -days 3650 -notext -md sha256 \
      -in intermediate/csr/intermediate.csr.pem \
      -out intermediate/certs/intermediate.cert.pem
chmod 444 intermediate/certs/intermediate.cert.pem

# Verify the certificate
openssl x509 -noout -text -in intermediate/certs/intermediate.cert.pem

# Verify the certificate against the CA
openssl verify -CAfile ca/certs/ca.cert.pem intermediate/certs/intermediate.cert.pem

# Create the certificate chain
cat intermediate/certs/intermediate.cert.pem ca/certs/ca.cert.pem > intermediate/certs/ca-chain.pem
chmod 444 intermediate/certs/ca-chain.pem
cp intermediate/certs/ca-chain.pem ..
