FROM rust:latest
WORKDIR .

COPY . .

#RUN cargo fetch e
EXPOSE 80
ARG build_or_run=build
# Chargement du profil 'release' défini dans le fichier Rocket.toml
RUN cargo ${build_or_run} --release 
