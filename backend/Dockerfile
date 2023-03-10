from rust:latest
WORKDIR .
COPY . .
RUN cargo fetch
EXPOSE 80
ARG ROCKET_ENV=release
CMD [ "cargo", "run", "--offline", "--release" ]