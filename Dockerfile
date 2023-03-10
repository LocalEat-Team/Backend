from rust:latest
WORKDIR .

COPY . .

#RUN cargo fetch e
EXPOSE 8000
# Chargement du profil 'release' défini dans le fichier Rocket.toml
#CMD [ "cargo", "run", "--release" ]

RUN if [[ -z "$build" ]] ; then cargo build --release ; else cargo run --release; fi