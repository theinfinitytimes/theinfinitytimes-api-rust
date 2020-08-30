FROM rust:latest

RUN mkdir /theinfinitytimes-api
COPY . /theinfinitytimes-api

WORKDIR /theinfinitytimes-api
RUN cargo build --release
RUN cargo install --path .

CMD ["theinfinitytimes-api-rs"]
