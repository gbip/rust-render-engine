[![Build Status](https://travis-ci.org/gbip/rust-render-engine.svg?branch=master)](https://travis-ci.org/gbip/rust-render-engine)

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

## Features impémentées

- [x] Support de la géomètrie à travers des fichiers .obj
- [x] Benchmarking du code
- [x] Rendu dans une résolution arbitraire
- [x] Choisir le sampling des rayons
- [x] Le support des textures
- [x] Le support des lumières
- [ ] Materiaux basés sur la physique

## Performances

Très mauvaises pour le moment...

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
- [x] Choose the subdivision sampling
- [x] Use textures
- [x] Support of lights
- [ ] Physic based materials

## Performances
The render engine has some terrible performances for the moment...
