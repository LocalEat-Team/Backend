from rust:latest
WORKDIR .

COPY ./src ./src
COPY ./.env ./.env
COPY ./Cargo.toml /Cargo.toml 
COPY ./Rocket.toml ./Rocket.toml

RUN cargo fetch
EXPOSE 80
# Chargement du profil 'release' défini dans le fichier Rocket.toml
CMD [ "cargo", "run", "--offline", "--release" ]