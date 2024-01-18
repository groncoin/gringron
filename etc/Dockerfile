# Multistage docker build, requires docker 17.05

# builder stage
FROM rust:1.45 as builder

RUN set -ex && \
    apt-get update && \
    apt-get --no-install-recommends --yes install \
    clang \
    libclang-dev \
    llvm-dev \
    libncurses5 \
    libncursesw5 \
    cmake \
    git

WORKDIR /usr/src/gringron

# Copying GrinGron
COPY . .

# Building GrinGron
RUN cargo build --release

# runtime stage
FROM debian:10

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y locales openssl

RUN sed -i -e 's/# en_US.UTF-8 UTF-8/en_US.UTF-8 UTF-8/' /etc/locale.gen && \
    dpkg-reconfigure --frontend=noninteractive locales && \
    update-locale LANG=en_US.UTF-8

ENV LANG en_US.UTF-8

COPY --from=builder /usr/src/gringron/target/release/gringron /usr/local/bin/gringron

WORKDIR /root/.gringron

RUN gringron server config && \
    sed -i -e 's/run_tui = true/run_tui = false/' gringron-server.toml

VOLUME ["/root/.gringron"]

EXPOSE 5633 5634 5635 5636

ENTRYPOINT ["gringron"]

CMD ["server", "run"]
