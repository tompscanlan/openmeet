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
    
    # Install Cassandra C++ Driver shared libraries
RUN cd /tmp && \
    wget https://datastax.jfrog.io/artifactory/cpp-php-drivers/cpp-driver/builds/2.17.1/e05897d/ubuntu/22.04/cassandra/v2.17.1/cassandra-cpp-driver_2.17.1-1_amd64.deb && \
    wget https://datastax.jfrog.io/artifactory/cpp-php-drivers/cpp-driver/builds/2.17.1/e05897d/ubuntu/22.04/cassandra/v2.17.1/cassandra-cpp-driver-dev_2.17.1-1_amd64.deb && \
    dpkg -i cassandra-cpp-driver_2.17.1-1_amd64.deb cassandra-cpp-driver-dev_2.17.1-1_amd64.deb && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y --no-modify-path --profile minimal --default-toolchain 1.80.1 --default-host x86_64-unknown-linux-gnu

COPY ./ /build

RUN cd /build && /root/.cargo/bin/cargo build --release && \
    objcopy --compress-debug-sections target/release/api ./main


ENV PATH=/root/.cargo/bin:$PATH
ENV RUST_VERSION=1.80.1
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8000

EXPOSE 8000

VOLUME [ "/build" ]
WORKDIR /build
CMD /build/main