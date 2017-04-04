/// Un fichier qui regroupe les différentes façons d'évaluer une intégrale à travers les
/// méthodes d'évaluation de Monte Carlo.

use math::{Vector2f, Vector3f};
use std::f32;
use rand::weak_rng;
use rand::distributions::{IndependentSample, Range};

/// Retourne un point dont les coordonnées sont aléatoires, selon une distribution uniforme, et
/// comprises entre -1 et 1.
fn generate_random_point() -> Vector2f {
    let range = Range::new(-1.0, 1.0);
    // weak_rng proviens de la crate rand
    let mut random_number_generator = weak_rng();
    Vector2f::new(range.ind_sample(&mut random_number_generator),
                  range.ind_sample(&mut random_number_generator))
}

/// Crée un échantillon sur le disque unité, avec une distribution uniforme.
/// * `u` - C'est un point dont chacune des coordonnées doit appartenir à [-1;1]
/// Renvoie un point en coordonnées polaires !
fn sample_disk_concentric(u: Vector2f) -> Vector2f {
    let offset: Vector2f = 2.0 * u - Vector2f::new(1f32, 1f32);
    if offset.x == 0.0 && offset.y == 0.0 {
        Vector2f::new(0.0, 0.0)
    } else {
        let theta: f32;
        let r: f32;
        if offset.x.abs() > offset.y.abs() {
            r = offset.x;
            theta = f32::consts::FRAC_PI_4 * (offset.y / offset.x);
        } else {
            r = offset.y;
            theta = f32::consts::FRAC_PI_2 - f32::consts::FRAC_PI_4 * (offset.x / offset.y);
        }
        r * Vector2f::new(f32::cos(theta), f32::sin(theta))
    }
}

/// Retourne un point en coordonnée sphérique distribués selon un cosinus (il a plus de chances
/// d'être au sommet de l'hémisphère qu'au bord).
/// Cette fonction projette les échantillons distribués uniformément issus de la fonction
/// `sample_disk_concentric` sur une demi-sphère. Le résultat étant des échantillons distribués
/// selon un cosinus. C'est la méthode de Malley.
/// * `u` - Un point dont les coordonnées sont aléatoires etcomprises entre -1 et 1.
fn generate_sample_cosine_hemisphere(u: Vector2f) -> Vector3f {
    let d = sample_disk_concentric(u);
    let z = f32::max(0f32, 1f32 - d.x * d.x - d.y * d.y).sqrt();
    Vector3f::new(d.x, d.y, z)
}

/// Retourne un point distribué selon une loi uniforme. L'hémisphère est orienté selon l'axe z.
/// * `u` - Un point dont les coordonnées sont aléatoires et comprises entre -1 et 1
fn generate_sample_uniform_hemisphere(u: Vector2f) -> Vector3f {
    let z: f32 = u.x;
    let r: f32 = f32::max(0.0, 1.0 - z * z).sqrt();
    let phi: f32 = 2.0 * f32::consts::PI * u.y;
    Vector3f::new(r * f32::cos(phi), r * f32::sin(phi), z)
}

/// Renvoie la densité de probabibilité pour un angle donnée.
fn probability_density_function_cosine_hemisphere(cos_theta: f32) -> f32 {
    cos_theta * f32::consts::FRAC_1_PI
}

/// Renvoie un vecteur de `samples` points distribuées de manière aléatoire sur l'hémisphère
/// centré en `u` et orienté selon `n`.
/// # Arguments
/// * `u` - un point autour duquel il faut générer les samples
/// * `n` - la normal au point `u`
/// * `samples` - le nombre de samples à générer
fn sample_cosine_hemisphere(u: &Vector3f, n: &Vector3f, samples: u32) -> Vec<Vector3f> {
    let mut result: Vec<Vector3f> = vec![];

    for _ in 1..samples {

        // Un point autour de la sphère unité.
        let sampled_point = generate_sample_cosine_hemisphere(generate_random_point());

        // On ramène le point autour de u.
        let corrected_sampled_point = &sampled_point + u;

        result.push(corrected_sampled_point);

    }
    result
}
