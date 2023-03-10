from rust:latest
WORKDIR .

COPY . .

#RUN cargo fetch e
EXPOSE 8000
# Chargement du profil 'release' défini dans le fichier Rocket.toml
RUN cargo run --release --target stable-x86_64-unknown-linux-gnu
