FROM parity/rust

WORKDIR /core
COPY . .
RUN cargo test
CMD cargo run --release