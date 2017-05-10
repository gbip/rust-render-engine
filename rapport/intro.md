# Introduction

Par la suite, les commandes bash seront prefixés par un $.

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

 * le langage est "verbeux"

Ainsi avoir choisis Rust nous a permis de drastiquement résoudre notre temps passer à débugger le programme, puisque les seuls erreurs que nous pouvions commettre étaient dues à des erreurs d'algorithmies.


# Notre méthode de travail

## Clippy

En plus du compilateur, nous avons utilisé [Clippy](https://github.com/Manishearth/rust-clippy). Il s'agit d'un analyseur statique de code qui ajoute 197 warnings au compilateur, allant de l'erreur d'algorithmie au respect des conventions de code.
Avoir un outil qui analyse notre code a été un gros avantage, puisque cela nous a permis d'avoir un code qui respecte à 100% la manière de penser du langage Rust.
Nous avons aussi pu éviter quelques erreurs d'innatentions avant l'execution.

## Git

Afin de pouvoir travailler collaborativement, nous avons utiliser le logiciel de gestionnaire de version `git`. Il s'agit d'un gestionnaire de version décentralisé.
Avec git chaque programmeur regroupe ses modifications en commits. Lorsque une ligne a été modifiée par plusieurs programmeurs, il y a conflit, et il faut le résoudre à la main.
Enfin le code se trouve sur un repertoire distant, ce qui permet d'assurer la synchronisation des versions à travers internet.


Vous pouvez accéder au répertoire distant du projet sur [github](https://github.com/gbip/rust-render-engine).

## Test unitaires

Afin de s'assurer du fonctionnement de chaque fonctionnalitée nous avons écris des tests unitaires au fur et à mesure du devellopement.
Le projet final comporte 20 tests unitaires, ce qui est peu, mais chaque test réalise en réalité plusieurs vérifications.

## Travis

A chaque fois que quelqu'un envoie des commits sur le repertoire distant, le service d'intégration continue Travis se met en route.
Celui-ci récupère le code et lance plusieurs commandes :
```
$ cargo fmt -- --write-mode=diff
$ cargo build
$ cargo test
```
La première commande vérifie que le code est bien formatté, elle quitte avec un code d'erreur différent de 0 si il est nécessaire de formatter le code.
La deuxième commande compile le code.
La troisième commande compile et lance les test unitaires.
Si jamais une de ces étapes échoue, nous recevons un mail, et les commits sont marqués comme échouant les test d'intégrations continues.
Il est possible de voir à tout moment le statut du projet [sur le site internet de Travis](https://travis-ci.org/gbip/rust-render-engine).

## Formatage du code

Afin d'avoir une base de code avec un style constant, nous utilisons l'outil [rustfmt](https://github.com/rust-lang-nursery/rustfmt).
Ce programme est lancé à travers cargo, et lors de son execution il va parcourir tous les fichiers sources et les formatter selon des règles de style définies dans un fichier.
Nous utilisons les règles de style par défaut.

Enfin, il est possible de mettre en place des script permettant de lancer cet outil automatiquement. Par exemple, sur Vim, rustfmt est lancé à chaque fois que l'on sauvegarde le buffer courant.
Sur Intellij IDEA il est possible de lancer le formattage du code avant toute compilation.

## Documentation

Nous avons essayé de documenter au maximum le projet. Malheuresement, la documentation est quand même très éparse.
En effet, nous avons beaucoup documenté le fonctionnement des fonctions via des commentaires décrivant les différentes lignes composant une fonction,
cependant il y a peu de documentation décrivant le fonctionnement du code en général.

## Compiler le projet, générer la documentation et lancer les test unitaires.

Pour installer le projet il faut commencer par installer [rustup](https://rustup.rs/).
```
$ curl https://sh.rustup.rs -sSf | sh
```

Ensuite vous pouvez télécharger le repertoire avec git, et compiler le projet.
```
$ sudo apt-get install git
$ git clone https://github.com/gbip/rust-render-engine
$ rustup override set nightly
$ cargo build --release
```

Pour lancer les test unitaires, il faut executer `cargo test` dans le repertoire du projet.
Pour compiler la documentation, il faut executer `cargo doc` dans le repertoire du projet.

## Langue des variables, du code et de l'interface
Nous sommes partis du principe que le standard, en informatique est l'anglais. Ainsi tous les noms de variables, de fonctions, de modules et de structure de données 
sont en anglais.
De plus, l'interface en ligne de commande est elle aussi en anglais.
Cependant, afin de faciliter leur rédaction, leur lecture et leur éventuelle compréhension, les commentaires sont en français.

# Scénario de fonctionnement
Nous avons choisi, pour des raisons de simplicité, de nous contacter d'une interface en ligne de commande pour l'interaction avec l'utilisateur.
## Interface de commande avec l'utilisateur
Lorsque l'on lance le logiciel sans argumment, un message d'aide s'affiche indiquant à l'utilisateur comment utiliser le logiciel.
C'est une pratique standard dans l'environnement UNIX:
```
$ ./render_engine
Usage : ./render_engine -g FILE -r FILE -w FILE
```

Détaillons les différents arguments :

 * > -g FILE or --generate FILE : Generate a template file in the location FILE for creating a scene

 Il s'agit là d'une option qui doit être utilisée toute seule : le logiciel est en mode "génération de structure" et ne cherchera même pas à charger le fichier de scène si il est spécifié.
 Les fichiers générés montrent comment créer une scène:

  * *template.json* : la description de la scène

  * *template_material_solid.json* : un exemple de matériau sans texture (couleur unie)

  * *template_material_texture.json* : un exemple de matériau avec une texture

 Nous détaillons dans la partie [Fichiers d'objets, de matériau et de scène] les différents fichiers nécessaires à la création d'une scène.


 * > -r FILE or --read FILE : Read FILE to load the scene before rendering. Needed for rendering, without a scene specified, the program will not render.

Cette option précise au logiciel quel fichier il doit lire pour créer la scène. Si cette option n'est pas spécifiée, le programme ne pourra pas lancer le rendu d'une scène.
Cette option est suffisante pour lancer la procédure de rendu, le logiciel sauvegardera alors l'image sous le nom 'untitled.png'

* > -w FILE or --write FILE : Write the output to FILE. The default is 'untitled.png'

Cette option indique au logiciel où doit être enregistrée l'image de sortie. Si cette option n'est pas présente, l'image sera enregistrée dans le fichier 'untitled.png'

## Fichiers d'objets, de matériau et de scène
La principale interaction avec l'utilisateur se fait au travers des fichiers décrivant respectivement, la scène, les matériau et les objets. Nous avons choisi d'utiliser le format [JSON](https://fr.wikipedia.org/wiki/JavaScript_Object_Notation) car il est plus simple à modifier que du XML, et est lui aussi lisible très facilement par un humain.
Enfin, il existe deux structures qui se retrouvent régulièrement dans les fichiers de scènes :

* Les coordonnées cartésiennes d'un point dans l'espace :
```json
"point": {
    "x": 0.0,
    "y": 0.0,
    "z": 5.0
}
```

* les différents composantes d'une couleur : il s'agit de quatres entiers compris entre 0 et 255 pour chaque composante Red Green Blue Alpha (transparence) RGBA :
```json
"background_color": {
	"r": 0,
    "g": 127,
    "b": 254,
    "a": 255
}
```
        
### La scène
Le fichier de scène correspond au fichier principal qui décris :

* la géométrie présente dans la scène, et le matériau qui y est affecté

* la ou les caméras présentent dans la scène

* la ou les lumières qui éclairent la scène

* les paramètres de rendu


Voici la description des différents champs qui composent ce fichier :

* *base_vector* : ce champ indique quels sont les trois vecteurs formant la base orthonormée pour représenter la géomètrie dans l'espace

* *cameras* : il s'agit d'une tableau de [Caméra]

* *objects*  : les différents objets  composant la scènne (géomètrie et matériau)

* *lights* : il s'agit d'un tableau de [Lumières]

* *renderer *  : les différents paramètres du rendu :
	* *res_x*,*res_y* : la résolution de l'image a calculer
	* *threads* : le nombre de coeurs à utiliser pour le calcul
	* *bucket_size* : taille des blocs subdivisant l'image pour la répartition du travail entre les coeurs
	* *sampler* : les paramètres de la génération des échantillons :
		* *HaltonSampler* ou *UniformSampler* permettent de choisir la méthode de génération des échantillons sur l'image 2D. *Haltonsampler* offre la meilleur qualité.
		* *subdivision_sampling* : le paramètre crucial qui va énormèment jouer sur la qualité de l'image finale. Il s'agit du nombre de rayons qui vont être lancés par pixel.
	* *filter* : les paramètres pour la reconstruction des pixels à partir des rayons :
		* *BoxFilter* ou *MitchellFilter* permettent de choisir quelle méthode utiliser lors du rendu. MichellFilter offre en théorie la meilleur qualité.
	* *background_color* : la color de fond lorsqu'aucun n'objet ne viens obstruer le rayon.


```json
{
  "world": {
    "base_vector": [],
    "cameras": [],
    "objects": [],
    "lights": []
  },
  "renderer": {
    "res_x": 960,
    "res_y": 540,
    "threads":8,
    "bucket_size":17,
    "sampler": {
      "HaltonSampler": {
        "subdivision_sampling": 4
      }
    },
    "filter": "BoxFilter",
    "background_color": {
      "r": 0,
      "g": 0,
      "b": 0,
      "a": 255
    }
  }
}
```

Le nombre de paramètre exposé est relativement important, et avec du recul certains n'ont pas leur place ici. 
Par exemple *base_vector* ne devrait même pas être exposé à l'utilisateur, c'est une convention que nous utilisons en interne.
De plus certains arguments pourrait être donnés en ligne de commande, comme le nombre de coeur à utiliser pour le calcul.


#### Caméra
La caméra est composé des champs suivants :
* *world_position* : la position de la caméra dans le monde. C'est le point à partir duquel on voit la scène.
* *target_position* : un point de l'espace vers lequel on regarde. Celui-ci est au centre de l'écran. Il permet d'orienter la caméra.
* *up* : un vecteur qui indique le 'haut' de l'image, utiliser pour faire tourner la caméra.
* *fov* : de l'acronyme 'Field of View', indique le champ de vision en degré de la caméra. Une valeur plus petite correspond à un effet de zoom.
* *clip* : la distance à partir de laquelle les rayons sont arrêtés par les objets. Dans notre exemple, si un objet se trouve à moins de 0.001 de la caméra, il ne sera pas visible.

```json
{
	"world_position": {
        "x": 0.0,
        "y": 0.0,
    	"z": 5.0
    },
	"target_position": {
        "x": 10.0,
        "y": 0.0,
    	"z": 5.0
    },
    "up": {
        "x": 0.0,
        "y": 0.0,
    	"z": 1.0
    },
    "fov": 70.0,
	"clip": 0.001
}
```


#### Objets


L'utilisation d'un objet fait appel à plusieurs fichiers : 

* le fichier qui contiens les informations de géométrie

* la section dans le fichier de scène qui indique la position de l'objet

* un matériau


##### Dans la scène

Ajouter un objet dans une scène consiste à spécifier les champs suivants :

* *position* : la position de l'objet.

* *scale* : un facteur d'échelle permettant de modifier la taille de l'objet selon plusieurs axes.

* *rotation* : la rotation de l'objet autour de chacun des axes.

* *obj_path* : le chemin vers un fichier .obj qui contiens les informations sur la géométrie.

* *visible* : ce booléen indique si l'objet est visible ou non. Si le booléen est à false, alors l'objet ne sera pas affiché.

* *material* : le chemin vers un fichier JSON indiquant les propriétés du matériau dont est fait l'objet. La description d'un matériau se trouve dans [Matériau]

```json 
{
	"position": {
        "x": 15.0,
        "y": 0.0,
    	"z": 5.0
    },
	"scale": {
        "x": 2.0,
        "y": 1.0,
    	"z": 2.0
    },
	"rotation": {
        "x": 0.0,
        "y": 0.0,
    	"z": 90.0
    },
    "obj_path": "models/plane_no_uv.obj",
    "name": "Example",
	"visible":false,
	"material" : "scenes/materials/solid_grey.json"
}
```

##### Format de stockage de la géomètrie

La géométrie est stockée sous le format [Wavefront Obj](https://fr.wikipedia.org/wiki/Objet_3D_(format_de_fichier)) qui est un format libre.
Un fichier .obj est un fichier texte, qui décris point par point, face par face, la géométrie d'un objet.
Nous ne supportons que les fonctionnalités 'de base' du standard Wavefront.
Notamment, il est impossible :
* de spécifier le matériau d'un objet dans le .obj
* de grouper plusieurs objets dans un .obj

Concrétement, il existe trois types de ligne que nous supportons :

* `o Plane` définit un nouvel objet.
* `v -0.500000 -0.500000 0.000000` définit un nouveau point de coordonnée (0.5,-0.5,0).
* `vt 0.999900 0.000100` définit des nouvelles coordonnées de textures dans le plan 2D.
* `vn 0.000000 0.000000 1.000000` définit un nouveau vecteur normal

* `f 2/1/1 4/2/1 3/3/1` définit un nouveau triangle composée des 3 points suivants :
	* le premier point a pour coordonnée spatiale le deuxième vertex *v* défini dans le fichier. Il a pour coordonnée de textures le premier point *vt* défini dans le fichier. Il a pour normale le premier vecteur de normal *vn* défini dans le fichier.
	* le deuxième point va chercher le 4éme *v* pour les coordonées spatiales, le deuxième *vt* pour les textures et le premier *vn* pour les normales
	* le troisième point a pour coordonée spatiale le troisième *v*, comme coordonée de texture le troisième *vt* et comme vecteur normal le premier *v*

Un exemple de fichier de géométrie est présent dans la partie [Exemple de fichier .obj].

A l'utilisation, la géométrie est générée par [Blender](https://www.blender.org/) car il est impensable d'écrire un fichier .obj à la main.

#### Lumières

Pour décrire une lumière il suffit de spécifier les trois champs suivants :

* *point* la position de la lumière

* *intensity* l'intensité de la lumière. Plus la valeur est grande, plus la source est lumineuse

* *color* la couleur de la lumière. Permet de créer des lumières de toutes les couleurs, même noires.

```json
{
	"point": {
        "position": {
            "x": 10.0,
            "y": 1.0,
        	"z": 8.0
        },
        "intensity": 10.0,
        "color" : {
            "r": 255,
            "g": 0,
            "b": 255,
        	"a": 255
    	}
	}
}
```

#### Matériaux

Le seul type de matériau implémenté actuellement dans le programme est un matériau qui se rapproche beaucoup du [matériau de Phong](https://fr.wikipedia.org/wiki/Ombrage_de_Phong).
Il est composé de trois couleurs ou textures qui represente chacune une composante spécifique du matériau : *ambient* , *diffuse* et *specular*.

Le fichier JSON d'un matériau se découpe aussi ainsi.A chaque champ est assignable au choix, une couleur unie, une texture ou une texture spéciale :
```json
{
  "diffuse": {
    "color": {
      "r": 204,
      "g": 29,
      "b": 20,
      "a": 255
    }
  },
  "specular": {
    "texture": {
      "map_path": "scenes/textures/checker_2k.jpg",
      "tiling_x": 10.0,
      "tiling_y": 10.0
    }
  },
  "ambient": {
     "normal":{}
  }
}
```

Ici, le champ *diffuse* se voit assigner une couleur unie, le champ *specular* une texture.
Pour la texture, il y a trois paramètres :

* *map_path* qui est le chemin vers le fichier d'image.

* *tiling_x* qui est le nombre de fois que la texture doit se répéter selon l'axe x.

* *tiling_y* qui est le nombre de fois que la texture doit se répéter selon l'axe y.

Le champ *ambient* se voit assigner la texture spéciale *normal* (la seule texture spéciale implémentée) qui va afficher le vecteur normal de l'objet en tout point de la surface.


## Fonctionnement général du programme
### Décodage des fichiers de scène
### Rendu
### Ecriture de l'image


# Implémentation
## Choix des structures de données

Pour faire fonctionner notre moteur de rendu, il est nécessaire de mettre en place une structure de données qui contiendra la géometrie 
des objets à rendre et leurs différentes caractéristiques. Cette structure de données doit permettre un accès efficace pour accélerer les
calculs, mais on doit aussi limiter son coût en mémoire. En effet, la scène à rendre peut contenir plusieurs millions de polygones.

Nous avons considéré qu'une scène est constitué de plusieurs objets, possèdant chacun une géométrie et un matériau. Nos scènes contenant peu
d'objets, il n'était pas nécessaire d'adopter une structure complexe pour les stocker. Nous avons donc utilisé une simple liste à accès direct
(type `Vec` en rust).

Pour la géométrie, nous avions les contraintes suivantes :
* les faces sont triangulaires, elles ont donc trois sommets
* chaque sommet contient des informations de textures et de normales.

Comme certaines faces peuvent partager un même sommet, nous avons d'abord envisager d'avoir une liste de faces et une liste de sommets, chaque face
faisant référence à ses trois sommets. Malheureusement, l'utilisation des références pose problème en Rust car le langage impose un contrôle
explicite de la mémoire. Plus particulièrement, dans ce cas le fait que la structure `Mesh` contienne des références vers certains de ses champs,
la rendait impossible à passer en paramètres.

## Dépendances

Nous avons 7 dépendances :

* *serde* nous permet de facilement sérialiser et déserialiser des structures de données.

* *image* nous permet de charger en mémoire des images, et d'écrire des images sur le disque.

* *getopts* nous permet de réaliser facilement l'analyse des arguments fourni au programme en ligne de commande.

* *num* rajoute des traits utiles pour manipuler des nombres de manière générique (libraire très peu utilisée au final).

* *colored* permet de facilement coloré les messages que l'on affiche sur la sortie standard.

* *scoped-pool* permet de mettre en place un groupe de thread qui vont travailler collaborativement sur la même tâche, et rajoute des garanties sur les threads.
Par exemple, scoped-pool permet de garantir au compilateur qu'un thread aura terminé de s'executer à la fin d'un bloc.

* *pbr* permet d'afficher une barre de progression dans le terminal.


## Description de chaque module
## Amélioration qualitatives
## Optimisations

# Quelques problèmes notables

# Annexes


## Exemple de fichier .obj



Le fichier suivant décris un plan avec des coordonées de textures :
```obj
o Plane
v -0.500000 -0.500000 0.000000
v 0.500000 -0.500000 0.000000
v -0.500000 0.500000 0.000000
v 0.500000 0.500000 0.000000
vt 0.999900 0.000100
vt 0.999900 0.999900
vt 0.000100 0.999900
vt 0.000100 0.000100
vn 0.000000 0.000000 1.000000
s off
f 2/1/1 4/2/1 3/3/1
f 1/4/1 2/1/1 3/3/1
```