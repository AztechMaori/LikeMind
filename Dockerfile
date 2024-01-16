FROM rust:latest

WORKDIR /server

COPY ./backend /server

RUN cargo install cargo-watch

EXPOSE 3000

CMD [ "cargo", "watch", "-x", "run" ]