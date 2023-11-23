# Rust as the base image
FROM rust:1.67

RUN USER=root cargo new --bin /usr/src/app
WORKDIR /usr/src/app

COPY ./ ./

# Install substreams binary
RUN LINK=$(curl -s https://api.github.com/repos/streamingfast/substreams/releases/latest | awk '/download.url.*linux_x86/ {print $2}' | sed 's/"//g') && curl -L $LINK | tar zxf -
RUN mv substreams /usr/local/bin
# Install substreams-sink-sql binary
RUN LINK=$(curl -s https://api.github.com/repos/streamingfast/substreams-sink-sql/releases/latest | awk '/download.url.*linux_x86/ {print $2}' | sed 's/"//g') && curl -L $LINK | tar zxf -
RUN mv substreams-sink-sql /usr/local/bin

# Set environment variables for buf the installation and do the install
ENV BIN="/usr/local/bin"
ENV VERSION="1.28.1"
RUN curl -sSL "https://github.com/bufbuild/buf/releases/download/v${VERSION}/buf-$(uname -s)-$(uname -m)" -o "${BIN}/buf" \
    && chmod +x "${BIN}/buf"

RUN make protogen
RUN make build

CMD ["make", "run"]
