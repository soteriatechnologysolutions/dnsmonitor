FROM rust:1.67

COPY . .

RUN cargo install --path .
CMD ["dnsmonitor"]