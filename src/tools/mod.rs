pub mod monte_carlo;

use math::{VectorialOperations, Vector3f};

/// Une fonction qui à partir de 2 vecteurs renvoies 2 nouveaux vecteurs orthogonaux selon le
/// procédé d'orthogonalisation de Schmidt (POS) .
/// # Arguments :
/// * `u` : Un vecteur qui ne sera pas changé
/// * `v` : Un  vecteur qui sera transformé selon le POS.
pub fn orthogonalize_vec(u: Vector3f, v: Vector3f) -> (Vector3f, Vector3f) {
    let v2 = u - u.dot_product_ref(&v) / u.dot_product_ref(&u) * u;
    (u, v2)
}
