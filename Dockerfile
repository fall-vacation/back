FROM rust:1.62.1

WORKDIR /usr/src/myapp
COPY . .
EXPOSE 8000

#RUN cargo install --path .
RUN cargo build --release

CMD ["./target/release/fall_vacation_back"]