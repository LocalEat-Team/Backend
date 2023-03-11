FROM rust:latest
COPY . .

EXPOSE 8000
RUN cargo build --release
# Chargement du profil 'release' défini dans le fichier Rocket.toml
CMD ["cargo", "run", "--release"]
