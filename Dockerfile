FROM rust:latest
WORKDIR .

COPY . .

#RUN cargo fetch e
EXPOSE 80
# Chargement du profil 'release' défini dans le fichier Rocket.toml
RUN cargo run --release 
