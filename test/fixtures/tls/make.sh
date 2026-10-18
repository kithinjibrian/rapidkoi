#!/bin/sh
# Makes the TLS fixtures for Rapidkoi's tests (PRP-009). Test-only: a
# throwaway CA that only the tests trust, through fetching.Network. Run from
# this folder: sh make.sh. Regenerating replaces every file, the CA too.
set -eu
days=36500
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes \
    -keyout ca.key -out ca.pem -days $days -subj "/CN=Rapidkoi test CA (tests only)" \
    -addext "basicConstraints=critical,CA:TRUE" -addext "keyUsage=critical,keyCertSign"
leaf() {  # name, subjectAltName, then extra options for the signing
    name=$1; san=$2; shift 2
    openssl req -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes \
        -keyout "$name.key" -out "$name.csr" -subj "/CN=$name"
    printf 'subjectAltName=%s\nbasicConstraints=CA:FALSE\nextendedKeyUsage=serverAuth\n' "$san" > "$name.ext"
    openssl x509 -req -in "$name.csr" -CA ca.pem -CAkey ca.key -CAcreateserial \
        -extfile "$name.ext" -out "$name.pem" "$@"
    rm "$name.csr" "$name.ext"
}
leaf localhost "DNS:localhost,IP:127.0.0.1" -days $days
leaf wrong-host "DNS:wrong.example" -days $days
leaf expired "DNS:localhost,IP:127.0.0.1" -not_before 20200101000000Z -not_after 20200102000000Z
# The CA's key isn't needed after signing: without it nothing new can be signed.
rm ca.key ca.srl
