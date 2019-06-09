FROM rust:1.35

RUN rustup toolchain install nightly
RUN rustup toolchain uninstall stable
RUN rustup default nightly

RUN mkdir /src
WORKDIR /src
ADD . /src/
RUN cargo build
RUN cargo test
RUN cargo install --path .
RUN rm -rf /src/
WORKDIR /
ADD ./static .
ADD readme.md .
ADD favicon.ico .
CMD ["rustrestapitest"]


