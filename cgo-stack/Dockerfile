FROM docker.io/golang:1.13-buster as builder

#COPY ./scripts/docker /etc/build
#RUN /etc/build/install-rust-compiler.sh
#ENV PATH="/root/.cargo/bin:${PATH}"

#COPY . /go/src/github.com/adrian-thurston/sandbox/cgo-stack
#WORKDIR /go/src/github.com/adrian-thurston/sandbox/cgo-stack

RUN apt-get update
RUN apt-get install --no-install-recommends -y \
	clang musl-tools valgrind gdb

# Install Rust for building Flux.
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain stable -y
RUN PATH="${HOME}/.cargo/bin:${PATH}" rustup target add x86_64-unknown-linux-musl

COPY . /go/src/github.com/adrian-thurston/sandbox/cgo-stack
WORKDIR /go/src/github.com/adrian-thurston/sandbox/cgo-stack
RUN make
RUN cp /go/src/github.com/adrian-thurston/sandbox/cgo-stack/cgo-stack /

FROM alpine:3.10
COPY --from=builder ./go/src/github.com/adrian-thurston/sandbox/cgo-stack/cgo-stack /cgo-stack
CMD ["/cgo-stack"]

