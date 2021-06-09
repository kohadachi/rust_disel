FROM rust:1.49

WORKDIR /rust_sapmle

RUN apt-get update -y && apt-get upgrade -y
RUN apt-get install -y mariadb-client

RUN cargo install diesel_cli --no-default-features --features mysql