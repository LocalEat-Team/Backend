# LocalEat BackEnd
Ce README est une documentation technique du backend de LocalEat, le point d'entrée du projet est ici : 
https://github.com/LocalEat-Team/LocalEat

## Architecture 
(La référence à cette architecture est dispo à la fin du fichier !)
 - main.rs : contient le main ainsi que l'ajout de toutes les routes créés au serveur
 - Dossier api  : Définition des routes (endpoint que va exploiter le front)
 - Dossier models : Définition des models de données exemple : user, product, etc...
 - Dossier repository : Définition de la logique côté base de données (MongoDb)

## Installation de l'environnement de développement
### Installation de RUST 
Lien : https://www.rust-lang.org/tools/install
Clique sur Download RUSTIP-INIT.EXE (64-BIT) puis exécute l'installateur.
Une fois fait, l'installateur à créé un dossier ".cargo\bin" dans "C:\Users\Fixe-Julien\.cargo\bin" (Ne soit pas bête, le path "Fixe-Julien" n'est pas le même chez toi ! )
Récupère ce path et ajoute le dans la variable d'environnement PATH.
Pour vérifier que RUST est bien installé, ouvre un CMD et tape "cargo". 

Maintenant que RUST est installé tu peux cloner ce dépôt et passer à la configuration de MongoDB 

### Configutation MongoDB :
Cette étape est nécessaire pour connecter le back à MongoDB au moement du lancement.

#### .env
Créer un fichier ".env" à la racine du projet, et y ajoute cette ligne :
```MONGOURI=mongodb+srv://<UTILISATEUR>:<PASSWORD>@clusterlocaleat.ywvwbpn.mongodb.net/?retryWrites=true&w=majority```

#### MongoDb
Ensuite connecte toi à MongoDb :
https://cloud.mongodb.com/v2/6405e6100b7bf303e93250be#/security/database/users
Normalement, tu dois avoir accès au cluster appelé "ClusterLocalEat". Si ce n'est pas le cas, check tos mails pour accepter l'invitation. 
 - Rendez-vous sur le cluster puis dans "Database Access".
 - Ajoute un nouvel utilisateur avec ton nom et prenom.
 - Génére un mdp et garde le quelque part.
 - Ajoute ton adresse Ip à MongoDB 

#### Connexion String
Retourne dans le fichier ".env" et modifie les champs UTILISATEUR et PASSWORD.
Tu peux à présent lancer le back !

### Vérifier la connexion 
Lance PostMan : 
 - Créé une requête POST à l'adresse : http://127.0.0.1:8000/user
 - Insere le body suivant : 
```
{
    "name":"Max",
    "location":"Le bouffon",
    "title":"un super titre"
}
```
 - Exécute la requête, Tu devrais avoir une réponse de ce genre : 
```
{
    "insertedId": {
        "$oid": "6406f56b3c62e50dafda280a"
    }
}
```
 - Rend toi sur MongoDb, dans le Cluster et dans "Collections", tu devrais voir ton nouvel utilisateur !
 
Félicitation tu as installé l'environnement de développement, maintenant au Boulot ! 
Signé Julien ton tyran préféré. 



Tuto de référence :
https://medium.com/geekculture/build-a-rest-api-with-rust-and-mongodb-rocket-version-7ea90ebd9fe7
