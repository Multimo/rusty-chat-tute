FROM rustlang/rust:nightly

WORKDIR /var/www/microservice/
COPY . .

RUN rustc --version
RUN cargo install

CMD ["microservice"]