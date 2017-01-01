use std::vec::Vec;
use math::{Vector2, Vector3, Vector3f};
use std::clone::Clone;
use render::{Color8};
use render;

/// This structs only hold references to the vertex that are stocked in the mesh.
#[derive(Clone)]
pub struct Triangle {
    pub vertex: [Vector3<f32>; 3],
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
        if vec.y < canvas.u.y {
            result |= BOTTOM;
        } else if vec.y > canvas.v.y {
            result |= TOP;
        }
        if vec.x < canvas.u.x {
            result |= LEFT;
        } else if vec.x > canvas.v.x {
            result |= RIGHT;
        }
        result
    }

    /// Returns a new line if the line described by p1 and p2 is clippable. If the line is not
    /// intersecting the box, return Some.
    pub fn clip_line(p1: Vector2f, p2: Vector2f, canvas: &render::Canvas) -> Option<(Vector2f,Vector2f)> {
        
        println!("call with :{} and {}",p1,p2);
        let outcode1 = computeOutcode(&p1, canvas);
        let outcode2 = computeOutcode(&p2, canvas);
        if !(outcode1 | outcode2) == 0b1111_1111 {
            return Some((p1, p2));
        } else if (outcode1 & outcode2) != 0b0000_0000 {
            return None;
        }
        let outcode_out: u8 = if outcode1 == IN { outcode2 } else {outcode1 };
        println!("outcode is : {:#b}", outcode_out);
        let mut x: f32 = 0_f32;
        let mut y: f32 = 0_f32;
        if (outcode_out & TOP) == TOP {
            x = p1.x + (p2.x - p1.x) * (canvas.v.y - p1.y) / (p2.y - p1.y);
            y = canvas.v.y;
        } else if (outcode_out & BOTTOM) == BOTTOM {
            x = p1.x + (p2.x - p1.x) * (canvas.u.y - p1.y) / (p2.y - p1.y);
            y = canvas.u.y;
        } else if (outcode_out & RIGHT) == RIGHT {
            y = p1.y + (p2.y - p1.y) * (canvas.v.x - p1.x) / (p2.x - p1.x);
            x = canvas.v.x;
        } else if (outcode_out & LEFT) == LEFT {
            y = p1.y + (p2.y - p1.y) * (canvas.u.x - p1.x) / (p2.x - p1.x);
            x = canvas.u.x;
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

        for elem in vertex_to_process.windows(2) {
            println!("{:?}",elem);
            //let line = cohen_sutherland::clip_line(elem[0],elem[1],canvas);

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

/// The standard Indexed Face Set data structure for mesh.
struct Mesh {
    triangle_list: Vec<Triangle>,
}

impl Mesh {

    fn new(vertex: Vec<Vector3f>,face_indices: Vec<(usize,usize,usize)>) -> Mesh {
        let mut mesh_triangles : Vec<Triangle> = vec!();
        for triangle in face_indices {
            let (v1,v2,v3) = triangle;
            mesh_triangles.push(Triangle{vertex:[vertex[v1],vertex[v2],vertex[v3]]});
        }
        Mesh{triangle_list:mesh_triangles}
    }
}

pub struct Object {
    // Maybe add a position field wich would acts as a global offset ?
    mesh: Mesh,
    color: Color8,
    position: Vector3f,
}

impl Object {
    pub fn get_triangles(&self) -> Vec<Triangle> {

        let mut result: Vec<Triangle> = vec![];
        for triangles in &self.mesh.triangle_list {
            result.push(triangles.clone());
        }
        return result;
    }
}

#[cfg(test)]
mod test {
    use render;
    use math::*;
    use super::cohen_sutherland::clip_line;

    const BOX : render::Canvas = render::Canvas{u:Vector2f{x:-2_f32,y:-2_f32},v:Vector2f{x:2_f32,y:2_f32}};
    const P1 : Vector2f = Vector2f{x:0_f32,y:0_f32};
    const P2 : Vector2f = Vector2f{x:4_f32,y:4_f32};
    const P3 : Vector2f = Vector2f{x:-8_f32,y:-8_f32};
    const P4 : Vector2f = Vector2f{x:-4_f32,y:1_f32};
    const P5 : Vector2f = Vector2f{x:4_f32,y:1_f32};
    const P6 : Vector2f = Vector2f{x:1_f32,y:6_f32};
    const P7 : Vector2f = Vector2f{x:1_f32,y:-7_f32};
    const P8 : Vector2f = Vector2f{x:1_f32,y:1_f32};
    const P9 : Vector2f = Vector2f{x:-1_f32,y:1_f32};
    const P10 : Vector2f = Vector2f{x:0_f32,y:3_f32};
    const P11 : Vector2f = Vector2f{x:6_f32,y:0_f32};
    const P12 : Vector2f = Vector2f{x:1_f32,y:4_f32};
    const P13 : Vector2f = Vector2f{x:2_f32,y:0_f32};
    #[test]
    fn test_line_clipping() {
        
        let (mut x,mut y) = match clip_line(P1,P2,&BOX) {
            None => (Vector2f::new(0_f32,0_f32),Vector2f::new(0_f32,0_f32)),
            Some(k) => k,
        };
        assert!(x==P1 && y == Vector2f{x:2_f32,y:2_f32});

        let (x,y) = match clip_line(P3,P2,&BOX) {
            None => (Vector2f::new(0_f32,0_f32),Vector2f::new(0_f32,0_f32)),
            Some(k)=> k,
        };
        assert!(x==BOX.u && y==BOX.v);

        let (x,y) = match clip_line(P4,P5,&BOX) {
            None => (Vector2f::new(0_f32,0_f32),Vector2f::new(0_f32,0_f32)),
            Some(k)=> k,
        };
        assert!(x==Vector2f::new(-2_f32,1_f32) && y==Vector2f::new(2_f32,1_f32));

        let (x,y) = match clip_line(P6,P7,&BOX) {
            None => (Vector2f::new(0_f32,0_f32),Vector2f::new(0_f32,0_f32)),
            Some(k)=> k,
        };
        assert!(x==Vector2f::new(1_f32,2_f32) && y==Vector2f::new(1_f32,-2_f32));
       
        let (x,y) = match clip_line(P8,P9,&BOX) {
            None => (Vector2f::new(0_f32,0_f32),Vector2f::new(0_f32,0_f32)),
            Some(k)=> k,
        };
        assert!(x==P8 && y==P9);
       
        let (x,y) = match clip_line(P2,P6,&BOX) {
            None => (Vector2f::new(0_f32,0_f32),Vector2f::new(0_f32,0_f32)),
            Some(k)=> k,
        };
        assert!(x==Vector2f::new(0_f32,0_f32) && y==Vector2f::new(0_f32,0_f32));
        
        let (x,y) = match clip_line(P10,P11,&BOX) {
            None => (Vector2f::new(0_f32,0_f32),Vector2f::new(0_f32,0_f32)),
            Some(k)=> k,
        };
        let k = Vector2f::new(2_f32,2_f32);
        assert!(x==k && y==k);
        
        let (x,y) = match clip_line(P12,P13,&BOX) {
            None => (Vector2f::new(0_f32,0_f32),Vector2f::new(0_f32,0_f32)),
            Some(k)=> k,
        };
        let k = Vector2f::new(1.5_f32,2_f32);
        assert!(x==k && y==P13);
    }

    #[test]
    fn test_triangle_clipping() {
        use render;
        use math::*;
        
        
        

    }
}
