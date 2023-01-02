FROM rust:1.66.0-bullseye as build

WORKDIR app

COPY ./src ./src
COPY ./.git ./.git
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./

RUN apt clean
RUN apt-get update --allow-releaseinfo-change -y
RUN apt upgrade -y
#RUN apt install build-essential cmake pkg-config libssl-dev clang -y

# Build your program for release
RUN cargo build --release

RUN mv target/release/res_dist ./bin
RUN strip bin

FROM debian:bullseye

WORKDIR app

RUN apt-get update --allow-releaseinfo-change -y
RUN apt upgrade -y
#RUN apt install build-essential cmake pkg-config libssl-dev clang -y

COPY --from=build /app/bin ./res_dist

# Run the binary
CMD ["./res_dist"]
