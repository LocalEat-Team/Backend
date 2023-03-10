from rust:latest
WORKDIR .

COPY . .

#RUN cargo fetch
EXPOSE 80
# Chargement du profil 'release' défini dans le fichier Rocket.toml
CMD [ "cargo", "run", "--offline", "--release" ]