FROM rustlang/rust:nightly as builder

WORKDIR /usr/src/pegasus-server

COPY . .

RUN cargo install --path .

FROM debian:stable-slim as production

# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

WORKDIR /pegasus-server

COPY --from=builder /usr/local/cargo/bin/pegasus-server /pegasus-server

COPY ./environments/ /pegasus-server/environments/

CMD ["/pegasus-server/pegasus-server"]
