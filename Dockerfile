FROM ubuntu:22.04

# Install Cassandra dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    libatk1.0-dev \
    libatk1.0-0 \
    libghc-gi-gdk-dev \
    libjavascriptcoregtk-4.0-dev \
    libcairo2-dev \
    libgdk-pixbuf-2.0-dev \
    libpango1.0-dev \
    libsoup2.4-dev \
    libuv1 \
    libuv1-dev \
    libwebkit2gtk-4.0-dev \
    openjdk-17-jdk \
    openssl \
    libssl-dev \
    wget \
    zstd \
    && rm -rf /var/lib/apt/lists/*
    
RUN cd /tmp && \
    wget https://datastax.jfrog.io/artifactory/cpp-php-drivers/cpp-driver/builds/2.17.1/e05897d/ubuntu/22.04/cassandra/v2.17.1/cassandra-cpp-driver_2.17.1-1_amd64.deb && \
    wget https://datastax.jfrog.io/artifactory/cpp-php-drivers/cpp-driver/builds/2.17.1/e05897d/ubuntu/22.04/cassandra/v2.17.1/cassandra-cpp-driver-dev_2.17.1-1_amd64.deb && \
    dpkg -i cassandra-cpp-driver_2.17.1-1_amd64.deb cassandra-cpp-driver-dev_2.17.1-1_amd64.deb

RUN cd /tmp && \
    wget https://sh.rustup.rs -O rustup.sh && \
    chmod +x rustup.sh && \
    ./rustup.sh -y --no-modify-path --profile minimal --default-toolchain 1.80.1 --default-host x86_64-unknown-linux-gnu && \
    . "/root/.cargo/env" && \ 
    echo 'source /root/.cargo/env' >> /root/.bashrc && \
    rustup default stable



ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:/root/.cargo/bin:$PATH \
    RUST_VERSION=1.80.1

# RUN set -eux; \
# curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh \

#     %%ARCH-CASE%%; \
#     url="https://static.rust-lang.org/rustup/archive/%%RUSTUP-VERSION%%/${rustArch}/rustup-init"; \
#     wget "$url"; \
#     echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
#     chmod +x rustup-init; \
#     ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host ${rustArch}; \
#     rm rustup-init; \
#     chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
#     rustup --version; \
#     cargo --version; \
#     rustc --version;


# RUN cd /tmp/ && \
#     wget https://datastax.jfrog.io/artifactory/cpp-php-drivers/cpp-driver/builds/2.17.1/e05897d/ubuntu/22.04/cassandra/v2.17.1/cassandra-cpp-driver-dev_2.17.1-1_amd64.deb && \
#     ar x cassandra-cpp-driver-dev_2.17.1-1_amd64.deb && \
#     cd / && \
#     tar xvf /tmp/data.tar.zst


VOLUME [ "/build" ]
# WORKDIR /build

# Start Cassandra and run cargo build
CMD cargo build --release