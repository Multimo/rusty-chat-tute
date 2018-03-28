FROM rustlang/rust:nightly

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

# Install vim for in-place editing.
RUN apt-get update -y && apt-get install -y vim

WORKDIR /var/www/rust-chat
COPY . .

RUN rustc --version
RUN cargo install

CMD ["rust-chat"]
