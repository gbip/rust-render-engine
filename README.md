[![Build Status](https://travis-ci.org/gbip/rust-render-engine.svg?branch=master)](https://travis-ci.org/gbip/rust-render-engine)
[![LoC](https://tokei.rs/b1/github/gbip/rust-render-engine?category=code)](https://github.com/gbip/rust-render-engine)
##### Pour compiler un fichier .md en pdf :
`pandoc intro.md -f markdown -t latex -o intro.pdf --toc`

# Français (English speakers see below)
## Compilation & Utilisation
### Compilation du projet
Il faut d'abord avoir [rustup](https://www.rustup.rs/) d'installé.
``` bash
git clone https://github.com/gbip/rust-render-engine
rustup override set nightly
cargo build --release
```
### Pour utiliser le logiciel
Pour générer un fichier d'exemple de scène `template.json`il faut lancer : `render_engine --generate` ou `render_engine -g`

Pour charger une scène et la rendre, il faut lancer : `render_engine --read <chemin_scene> --write <chemin_fichier>.png`

## Features implémentées

- [x] Support de la géomètrie à travers des fichiers .obj
- [x] Benchmarking du code
- [x] Rendu dans une résolution arbitraire
- [x] Choix de la méthode d'échantillonnage des rayons
- [x] Choix de la méthode de filtrage des échantillons récoltés
- [x] Le support des textures
- [x] Le support des lumières
- [ ] Materiaux basés sur la physique

## Performances

Le moteur de rendu n'est pour l'instant pas très performant...
Benchmark à venir.

# English
## Compiling & Using
### Compiling the project
First, you need to make sure [rustup](https://www.rustup.rs/) is installed.
``` bash
git clone https://github.com/gbip/rust-render-engine
rustup override set nightly
cargo build --release
```
### How to use
In order to generate an example file for a scene called `template.json`you can run : `render_engine --generate` or `render_engine -g`

In order to load a scene from a file, and save it to a .png, you can run : `render_engine --read <path_scene> --write <path_file>.png`

## Implemented features

- [x]  Arbitrary geometry support through .obj files (wavefront specification)
- [x] Benchmarks for the code
- [x] Rendering in an arbitrary resolution
- [x] Choose the sampler
- [x] Choose the filter
- [x] Support of textures
- [x] Support of lights
- [ ] Physic based materials

## Performances
The render engine is currently quite slow...
Benchmarking incoming.
