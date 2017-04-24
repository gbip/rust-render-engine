# Introduction

## L'histoire du raytracing

Le raytracing est une technique devellopée dans les anénes 60 permettant la synthèse d'images par un ordinateur.
Cette technique a connu un boom dans les années 90 pour permettre la réalisation d'effets spéciaux dans le cinéma.
C'est une méthode désormais utilisée dans de nombreux domaines : prévisualisation architecturale, cinéma, dessin animé, publicité, etc.

## L'interêt de ce projet pour nous 

Les enjeux autour du raytracing sont très importants, il existe toute une industrie organisée autour de cette technique de rendu, avec plusieurs millions de personnes qui utilisent chaque jour des moteurs de rendu en lancer de rayon.
De plus c'est un sujet qui nous interressait personellement, puisque nous avons déjà tous deux utilisé des moteurs de rendus (Cycle avec Blender, Vray avec 3DSMax).
Enfin, le sujet se prêtait particulièrement bien au cadre de ce projet, puisque il est très motivant.
En effet, nous avons directement un retour sur investissement à travers les images qui sortent directement de notre moteur de rendu.

## Pourquoi Rust

Dès le départ nous avons voulu partir sur un langage de programmation système, afin d'obtenir le maximum de performances.
Le seul langage sur ce créneau est le C/C++, cependant nous avons déjà eu plusieurs experiences tous les deux avec C++, et le fait que la langage soit très permissif à la compilation, retardant les bugs à l'execution nous dérangeait.
Cependant, depuis 2015 il existe un nouveau langage, poussé par Mozilla, nommé Rust.

> Rust est un langage de programmation compilé multi-paradigme conçu et développé par Mozilla Research. Il a été conçu pour être « un langage sécurisé, concurrent, pratique », supportant les styles de programmation purement fonctionnel, modèle d'acteur, procédural et orienté objet.
[Wikipedia](https://fr.wikipedia.org/wiki/Rust_(langage))

Les principaux points forts de Rust sont :

 * La sûreté de la mémoire est assurée à la compilation, il n'existe pas de `segfault` en Rust.

 * La concurrence est gerée de manière interne au langage: il n'existe pas de course au donnée en Rust. Ce point nous a été très utile pour paralléliser le moteur de rendu.

 * L'écosystème moderne (gestionnaire de paquet, compilateur, libraire standard, etc.). En effet, pour installer une libraire il suffit d'ajouter une ligne: toutes les dépendances sont gerées par `cargo` qui est le gestionnaire de projet de Rust. Un autre avantage de l'écosystème moderne est le compilateur, qui contrairement à g++/clang++ (deux compilateurs C++) offre des erreurs compréhensibles, et avec cette erreur, la méthode pour la résoudre.


Il existe aussi des points faibles vis à vis de ce langage, la plupart découlant de la jeunesse du langage :

 * Les librairies sont généralement encore jeune et pas toujours stable.

 * La compilation est lente, le compilateur ne parallèlise pas les tâches.

Ainsi avoir choisis Rust nous a permis de drastiquement résoudre notre temps passer à débugger le programme, puisque les seuls erreurs que nous pouvions commettre étaient dues à des erreurs d'algorithmies.


## Notre méthode de travail

### Clippy

En plus du compilateur, nous avons utilisé [Clippy](https://github.com/Manishearth/rust-clippy). Il s'agit d'un analyseur statique de code qui ajoute 197 warnings au compilateur, allant de l'erreur d'algorithmie au respect des conventions de code.
Avoir un outil qui analyse notre code a été un gros avantage, puisque cela nous a permis d'avoir un code qui respecte à 100% la manière de penser du langage Rust.
Nous avons aussi pu éviter quelques erreurs d'innatentions avant l'execution.

### Git

Afin de pouvoir travailler collaborativement, nous avons utiliser le logiciel de gestionnaire de version `git`. Il s'agit d'un gestionnaire de version décentralisé.
Avec git chaque programmeur regroupe ses modifications en commits. Lorsque une ligne a été modifiée par plusieurs programmeurs, il y a conflit, et il faut le résoudre à la main.
Enfin le code se trouve sur un repertoire distant, ce qui permet d'assurer la synchronisation des versions à travers internet.


Vous pouvez accéder au répertoire distant du projet sur [github](https://github.com/gbip/rust-render-engine).

### Test unitaires

Afin de s'assurer du fonctionnement de chaque fonctionnalitée nous avons écris des tests unitaires au fur et à mesure du devellopement.
Le projet final comporte 20 tests unitaires, ce qui est peu, mais chaque test réalise en réalité plusieurs vérifications.




### Travis

### Formatage du code

### Documentation

## Compiler le projet

Pour installer le projet il faut commencer par installer [rustup](https://rustup.rs/).
```
curl https://sh.rustup.rs -sSf | sh
```

Ensuite vous pouvez télécharger le repertoire avec git, et compiler le projet.
```
sudo apt-get install git
git clone https://github.com/gbip/rust-render-engine
rustup override set nightly
cargo build --release
```
