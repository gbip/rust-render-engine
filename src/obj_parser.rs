use obj3D::{Triangle, Mesh};
use std::fs::File;
use std::io::{BufRead, BufReader};
use math::{Vector2f, Vector3f};

// The Raw Point represents a triangle point where each coordinate is an index to the real value
// stored in a vector
#[derive(Debug)]
struct RawPoint(usize, usize, Option<usize>);

// The Raw Data represents a a type of value for each points of the triangle. It could be position,
// normals or textures.
struct RawData(u32, u32, u32);

// A simple structure to hold data before initializing the triangle.
#[derive(Debug)]
struct RawTriangle(RawPoint, RawPoint, RawPoint);


enum LineType {
    Ignore,
    Face(RawData, RawData, Option<RawData>),
    Vertex(f32, f32, f32),
    Normal(f32, f32, f32),
    TexCoord(f32, f32),
}

// A function to convert an Option<(u32,u32,u32)> to (Option<usize>,..).
// Useful for parsing texture coordinates
fn propagate_option(val: Option<RawData>) -> (Option<usize>, Option<usize>, Option<usize>) {
    match val {
        Some(tuple) => (Some(tuple.0 as usize), Some(tuple.1 as usize), Some(tuple.2 as usize)),
        None => (None, None, None),
    }
}

// Open an obj file and return a mesh with the data.
pub fn open_obj(file: &str) -> Mesh {

    let reader = BufReader::new(open_obj_file(file));

    let mut tris: Vec<(RawData, RawData, Option<RawData>)> = vec![];
    let mut pos: Vec<Vector3f> = vec![];
    let mut normals: Vec<Vector3f> = vec![];
    let mut tex: Option<Vec<Vector2f>> = None;

    // We clean the reader of all useless lines before iterating over it.
    for line in &reader.lines()
        .map(|l| l.expect("Error while reading line"))
        .collect::<Vec<String>>() {
        let parsed_line = match parse_line(line) {
            Ok(t) => t,
            Err(e) => panic!(e),
        };

        // We parse the file line by line and fill the vectors
        match parsed_line {
            LineType::Ignore => continue,
            LineType::Face(pos, norm, tex) => tris.push((pos, norm, tex)),
            LineType::Normal(x, y, z) => normals.push(Vector3f::new(x, y, z)),
            LineType::Vertex(x, y, z) => pos.push(Vector3f::new(x, y, z)),
            LineType::TexCoord(u, v) => {
                match tex {
                    Some(ref mut vec) => {
                        vec.push(Vector2f::new(u, v));
                    }
                    None => {
                        tex = Some(vec![Vector2f::new(u, v)]);
                    }
                }
            }
        };
    }

    let mut mesh = Mesh::new_empty();
    let mut raw_triangles: Vec<RawTriangle> = vec![];

    // We just convert the Option<RawData> to a (Option<usize>,...,...)
    #[allow(type_complexity)]
    let tris: Vec<(RawData, RawData, (Option<usize>, Option<usize>, Option<usize>))> =
        tris.into_iter()
            .map(|t| (t.0, t.1, propagate_option(t.2)))
            .collect::<Vec<(RawData, RawData, (Option<usize>, Option<usize>, Option<usize>))>>();
    for t in tris {
        let (p1, p2, p3) = (RawPoint((t.0).0 as usize, (t.1).0 as usize, (t.2).0),
                            RawPoint((t.0).1 as usize, (t.1).1 as usize, (t.2).1),
                            RawPoint((t.0).2 as usize, (t.1).2 as usize, (t.2).2));
        raw_triangles.push(RawTriangle(p1, p2, p3))
    }

    add_triangles(&mut mesh, &raw_triangles, &pos, &normals, &tex);
    mesh
}

fn add_triangles(mesh: &mut Mesh,
                 tris: &[RawTriangle],
                 pos: &[Vector3f],
                 norm: &[Vector3f],
                 tex: &Option<Vec<Vector2f>>) {
    for triangle in tris {
        let p1 = &triangle.0;
        let p2 = &triangle.1;
        let p3 = &triangle.2;
        let u = Mesh::create_point(p1.0, p1.1, p1.2, pos, norm, tex);
        let v = Mesh::create_point(p2.0, p2.1, p2.2, pos, norm, tex);
        let w = Mesh::create_point(p3.0, p3.1, p3.2, pos, norm, tex);
        mesh.add_triangle(Triangle::new(u, v, w));
    }
}


//Split a given line and parse each float value inside.
fn get_floats(line: &str) -> Vec<f32> {
    //We split the string by the whitespaces | parse each substring as a f32 | throw away
    //things that doesn't parse to f32
    line.split_whitespace()
        .filter_map(|val: &str| val.parse::<f32>().ok())
        .collect()
}

// Input : " 2//1 4//1 3//1"
// Output : [["2", "1"], ["4", "1"], ["3", "1"]]
fn get_face(str: &str) -> Vec<Vec<String>> {
    str.split(' ').map(|x| x.split('/') // we split the line by the '/' character
                            .map(|x| x.to_string()) // we convert the char to a string
                            .filter(|x| x!="") // we remove the empty strings
                            .collect())
                  .filter(|x| x[0]!="f") // we remove useless junk
                  .collect()

}

fn convert_to_u32(string: &str) -> u32 {
    str::parse::<u32>(string).expect("Error while parsing integer indices")
}


// Take the first 3 elements of a vector, and returns a RawData tuple.
fn make_tuple(vec: &[u32]) -> RawData {
    RawData(vec[0], vec[1], vec[2])
}

// We know two things : either there is position + normal, or there is position + normal +
// textures. Plus, we only have vertex per triangle.
fn extract_indexes(line: &str) -> Result<(RawData, RawData, Option<RawData>), String> {
    let data = get_face(line);
    let mut id_pos: Vec<u32> = vec![];
    let mut id_norm: Vec<u32> = vec![];
    let mut id_tex: Vec<u32> = vec![];

    //Represents the fact that a line may contain an invalid number of indices
    let mut error = false;
    let parsed_data: Vec<Vec<u32>> = data.iter()
        .map(|u| {
            u.iter()
                .map(|val| convert_to_u32(val))
                .collect()
        })
        .collect();

    //Splitting the case between the position + normal or the pos + norm + tex scenario
    for elem in parsed_data {
        match elem.len() {
            3 => {
                id_pos.push(elem[0]);
                id_tex.push(elem[1]);
                id_norm.push(elem[2])
            }
            2 => {
                id_pos.push(elem[0]);
                id_norm.push(elem[1]);
            }
            _ => {
                error = true;
            }
        }
    }

    if error {
        Err(format!("Incorrect number of indices, line : {}", line))
    } else {
        Ok((make_tuple(&id_pos),
            make_tuple(&id_norm),
            match id_tex.len() {
                3 => Some(make_tuple(&id_tex)),
                _ => None,
            }))
    }
}


fn parse_normal(line: &str) -> Result<LineType, String> {
    //We clone the line, to use line after for debugging.
    let floats = get_floats(line);
    if floats.len() == 3 {
        Ok(LineType::Normal(floats[0], floats[1], floats[2]))
    } else {
        Err(format!("Invalide number of float value, expected 3, found : {} | Line parsed : \
                     {} ",
                    floats.len(),
                    line))
    }
}

fn parse_vertex(line: &str) -> Result<LineType, String> {
    match parse_normal(line) {
        Ok(LineType::Normal(u, v, w)) => Ok(LineType::Vertex(u, v, w)),
        Err(t) => Err(t),
        _ => unreachable!(),
    }
}

// Get the first 3 elements from a vector and returns them in a tuple
fn vec_to_tuple3(vec: &[u32]) -> (u32, u32, u32) {
    (vec[0], vec[1], vec[2])
}

fn parse_face(line: &str) -> Result<LineType, String> {
    let indices = extract_indexes(line);
    match indices {
        // Transform the vector into a tuple3
        Ok(i) => Ok(LineType::Face(i.0, i.1, i.2)),
        //In cas of an error, we just propagate it one level further
        Err(e) => Err(e),
    }
}

fn parse_tex_coord(line: &str) -> Result<LineType, String> {
    let floats = get_floats(line);
    if floats.len() == 2 {
        Ok(LineType::TexCoord(floats[0], floats[1]))
    } else {
        Err(format!("Invalide number of float value, expected 3, found : {} | Line parsed : \
                     {} ",
                    floats.len(),
                    line))
    }
}


fn parse_line(line: &str) -> Result<LineType, String> {
    match line.chars().nth(0).expect("Error while reading line") {
        //Trivial cas, we just doesn't support groups, or individual objects
        'o' | 'g' | '#' | 'u' | 'm' | 's' => Ok(LineType::Ignore),
        'f' => parse_face(line),
        'v' => {
            match line.chars().nth(1).expect("Error while reading line") {
                ' ' => parse_vertex(line),
                'n' => parse_normal(line),
                't' => parse_tex_coord(line),
                _ => Err("Unexpected symbol".to_string()),
            }
        }
        _ => Err("Unexpected symbol".to_string()),
    }
}

fn open_obj_file(path: &str) -> File {
    match File::open(path) {
        Ok(t) => t,
        Err(e) => panic!("Error while trying to open the file: {} - {}", path, e),
    }
}

#[cfg(test)]
mod test {
    use math::{Vector3, Vector3f};
    use super::super::*;

    #[test]
    fn test_obj_parsing_plane() {
        let a: f32 = 8.555269;
        let b: f32 = 5.030090;
        let c: f32 = 4.669442;
        let d: f32 = 10.555269;
        let e: f32 = 2.669442;

        let norm1: Vector3f = Vector3::new(0.0, 1.0, 0.0);
        let p1 = GeoPoint::new(Vector3::new(a, b, c), norm1, None);
        let p2 = GeoPoint::new(Vector3::new(d, b, c), norm1, None);
        let p3 = GeoPoint::new(Vector3::new(a, b, e), norm1, None);
        let p4 = GeoPoint::new(Vector3::new(d, b, e), norm1, None);

        let t1 = Triangle::new(p2, p4, p3);
        let t2 = Triangle::new(p1, p2, p3);

        let expected_result = Mesh { triangles: vec![t1, t2] };

        assert_eq!(obj_parser::open_obj("models/plane_no_uv.obj"),
                   expected_result);
    }
}
