use std::vec::Vec;
use math::{Vector2, Vector3, Vector3f};
use std::clone::Clone;
use render::{Color8};
use render;

/// This structs only hold references to the vertex that are stocked in the mesh.
#[derive(Clone)]
pub struct Triangle<'a> {
    pub vertex: [&'a Vector3<f32>; 3],
}

pub struct Polygon<'a> {
    vertex: Vec<&'a Vector3<f32>>,
}

pub trait IsPolygon<'a> {
    fn find_points_in_polygon(&'a self, sizeX: u64, sizeY: u64) -> Vec<Vector2<u16>>;
}

impl<'a> IsPolygon<'a> for Polygon<'a> {
    fn find_points_in_polygon(&'a self, sizeX: u64, sizeY: u64) -> Vec<Vector2<u16>> {
        vec![]
    }
}

struct Triangle2D {
    vertex: [Vector2<f32>; 2],
}

/// Cohen-Sutherland line clipping algorithm.
mod cohen_sutherland {
    use math::{Vector2f};
    use render::{Color8};
    use render;
    
    const IN: u8 = 0b0000_0000;
    const LEFT: u8 = 0b0000_0001;
    const RIGHT: u8 = 0b0000_0010;
    const BOTTOM: u8 = 0b0000_0100;
    const TOP: u8 = 0b0000_1000;

    /// Compute the outcode of vec for the canvas given in argument.
    fn computeOutcode(vec: &Vector2f, canvas: &render::Canvas) -> u8 {
        let mut result = IN;
        if vec.x < canvas.u.x {
            result |= BOTTOM;
        } else if vec.x > canvas.v.x {
            result |= TOP;
        }
        if vec.y < canvas.u.y {
            result |= LEFT;
        } else if vec.y > canvas.u.y {
            result |= RIGHT;
        }
        result
    }

    // Returns a new line if the line described by p1 and p2 is clippable. If the line is not
    /// intersecting the box, return Some.
    fn clip_line(p1: Vector2f, p2: Vector2f, canvas: &render::Canvas) -> Option<[Vector2f; 2]> {

        let outcode1 = computeOutcode(&p1, canvas);
        let outcode2 = computeOutcode(&p2, canvas);
        if !(outcode1 | outcode2) == 0b1111_1111 {
            return Some([p1, p2]);
        } else if (outcode1 & outcode2) == 0b1111_1111 {
            return None;
        }

        let outcode_out: u8 = if outcode1 == IN { outcode2 } else { outcode1 };
        let mut x: f32 = 0_f32;
        let mut y: f32 = 0_f32;

        if (outcode_out & TOP) == 0b1111_1111 {
            let x = p1.x + (p2.x - p1.x) * (canvas.v.y - p1.y) / (p2.y - p1.y);
            let y = canvas.v.y;
        } else if (outcode_out & BOTTOM) == 0b1111_1111 {
            let x = p1.x + (p2.x - p1.x) * (canvas.u.y - p1.y) / (p2.y - p1.y);
            let y = canvas.u.y;
        } else if (outcode_out & RIGHT) == 0b1111_1111 {
            let y = p1.y + (p2.y - p1.y) * (canvas.v.x - p1.x) / (p2.x - p1.x);
            let x = canvas.v.x;
        } else if (outcode_out & LEFT) == 0b1111_1111 {
            let y = p1.y + (p2.y - p1.y) * (canvas.u.x - p1.x) / (p2.x - p1.x);
            let x = canvas.u.x;
        }

        // Now that we have processed one point, we do an other pass, in case of we need to
        // process the points again.
        if outcode_out == outcode1 {
            return clip_line(Vector2f { x: x, y: y }, p2, canvas);
        } else {
            return clip_line(p1, Vector2f { x: x, y: y }, canvas);
        }
    }
}

impl Triangle2D {
    /// Implementation of the Sutherland-Hodgman algorithm for clipping triangles
    fn trim_to_canvas(self, canvas: &render::Canvas) -> Polygon2D {
        let result = Polygon2D::new();
        let vertex_to_process = vec![self.vertex];

        for vertex in vertex_to_process.windows(2) {



        }

        result
    }
}

struct Polygon2D {
    pub vertex: Vec<Vector2<f32>>,
}

impl Polygon2D {
    fn new() -> Polygon2D {
        Polygon2D { vertex: vec![] }
    }
    fn add_new_vertex(&mut self, vertex: Vector2<f32>) {
        self.vertex.push(vertex);
    }
}

/// The standard INdexed Face Set data structure for mesh.
struct Mesh<'a> {
    vertex_list: Vec<Vector3<f32>>,
    triangle_list: Vec<Triangle<'a>>,
}

pub struct Object<'a> {
    // Maybe add a position field wich would acts as a global offset ?
    mesh: Mesh<'a>,
    color: Color8,
    position: Vector3f,
}

impl<'a> Object<'a> {
    pub fn get_triangles(&self) -> Vec<Triangle> {

        let mut result: Vec<Triangle> = vec![];
        for triangles in &self.mesh.triangle_list {
            result.push(triangles.clone());
        }
        return result;
    }
}
