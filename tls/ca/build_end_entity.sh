#!/bin/bash

DOMAIN=$1
COUNTY_CODE=DE
COUNTRY=Gernmany

# Without password for the key
openssl genrsa -out intermediate/private/$DOMAIN.key.pem 2048
# With password for the key
#openssl genrsa -aes256 -out intermediate/private/www.example.com.key.pem 2048
chmod 400 intermediate/private/$DOMAIN.key.pem

# Create the CSR
#openssl req -config openssl_intermediate.cnf \
openssl req -config openssl.cnf \
      -subj "/C=$COUNTY_CODE/L=$COUNTRY/CN=$DOMAIN" \
      -key intermediate/private/$DOMAIN.key.pem \
      -new -sha256 -out intermediate/csr/$DOMAIN.csr.pem

# Sign the certificate
#openssl ca -config openssl_intermediate.cnf \
openssl ca -config openssl.cnf \
      -extensions redhac_cert -days 375 -notext -md sha256 \
      -in intermediate/csr/$DOMAIN.csr.pem \
      -out intermediate/certs/$DOMAIN.cert.pem
chmod 644 intermediate/certs/$DOMAIN.cert.pem

# Verify the certificate
openssl x509 -noout -text -in intermediate/certs/$DOMAIN.cert.pem

# Verify the certificate chain of trust
openssl verify -CAfile intermediate/certs/ca-chain.pem intermediate/certs/$DOMAIN.cert.pem

# Copy certs in place
cat intermediate/certs/$DOMAIN.cert.pem > ../$DOMAIN.cert.pem
cat intermediate/certs/ca-chain.pem >> ../$DOMAIN.cert.pem
#cp intermediate/certs/$DOMAIN.cert.pem ..
cp intermediate/private/$DOMAIN.key.pem ..
