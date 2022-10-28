FROM rust:1.64.0 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/fall_vacation_back /usr/local/bin/fall_vacation_back
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["fall_vacation_back"]