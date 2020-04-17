FROM rust:latest as builder
WORKDIR /usr/src/myapp
RUN rustup default nightly \
 && rustup update
ENV ROCKET_ENV=production
ENV DB_URL=mongodb://mongo:27017
ARG JWT_KEY
RUN cargo install connect4-server --git 'https://github.com/Arunscape/ECE421-GroupProject1.git'
RUN git clone https://github.com/Arunscape/ECE421-GroupProject1.git
WORKDIR /usr/src/myapp/ECE421-GroupProject1/connect4/connect4-web
ENV SERVER_URL=connect4.woosaree.xyz
RUN cargo install wasm-pack
RUN curl -sL https://deb.nodesource.com/setup_13.x | bash -
RUN apt-get install -y nodejs
RUN npm i -g rollup
RUN ./build
RUN pwd
RUN ls -la

FROM rust:slim-stretch
#RUN addgroup -S rust && adduser -S -G rust rust
#RUN chown -R rust /usr/local/bin
WORKDIR /connect4/connect4-server
RUN mkdir -p /connect4/connect4-web/pkg
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
COPY --from=builder /usr/src/myapp/ECE421-GroupProject1/connect4/connect4-web/index.html /connect4/connect4-web
COPY --from=builder /usr/src/myapp/ECE421-GroupProject1/connect4/connect4-web/main.js /connect4/connect4-web
COPY --from=builder /usr/src/myapp/ECE421-GroupProject1/connect4/connect4-web/pkg /connect4/connect4-web/pkg
RUN chmod +x /usr/local/bin/server
#USER rust
ENV ROCKET_ENV=production
EXPOSE 8000
CMD ["server"]
