FROM rust:slim-bullseye as build

RUN apt-get update && apt-get install -y \
    build-essential autoconf automake libtool m4 \
    libssl-dev pkg-config

WORKDIR "/symsyq"

RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
COPY Cargo.toml ./
RUN cargo build --release

COPY . .
RUN cargo build --release

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y python3-pip ffmpeg
RUN pip install -U yt-dlp

COPY --from=build /symsyq/target/release/symsyq .

CMD ["./symsyq"]
