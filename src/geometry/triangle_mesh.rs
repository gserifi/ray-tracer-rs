use crate::geometry::{accel::AABB, Bvh, HitRecord, Hittable};
use crate::materials::Material;
use crate::optics::Ray;
use crate::utils::{Interval, Point3, Vec3, Vec3Ext};
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct Triangle {
    vertices: [Point3; 3],
    normals: [Vec3; 3],
    uvs: [Point3; 3],

    mat: Rc<dyn Material>,
    bbox: AABB,
}

impl Triangle {
    pub fn new(
        vertices: [Point3; 3],
        normals: [Vec3; 3],
        uvs: [Point3; 3],
        mat: Rc<dyn Material>,
    ) -> Self {
        let bbox = AABB::wrap_triangle(&vertices);

        Self {
            vertices,
            normals,
            uvs,
            mat,
            bbox,
        }
    }
}

impl From<Triangle> for Box<dyn Hittable> {
    fn from(triangle: Triangle) -> Self {
        Box::new(triangle)
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, t, rec) {
            return false;
        }

        rec.debug.inc_intersection_checks();

        let e1 = self.vertices[1] - self.vertices[0];
        let e2 = self.vertices[2] - self.vertices[0];
        let pvec = r.direction().cross(&e2);

        let det = e1.dot(&pvec);

        if det.abs() < 1e-6 {
            return false;
        }

        let inv_det = 1.0 / det;

        let tvec = r.origin() - self.vertices[0];
        let u = tvec.dot(&pvec) * inv_det;

        if u < 0.0 || u > 1.0 {
            return false;
        }

        let qvec = tvec.cross(&e1);
        let v = r.direction().dot(&qvec) * inv_det;

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let root = e2.dot(&qvec) * inv_det;

        if !t.surrounds(root) {
            return false;
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let normal = (self.normals[0] * (1.0 - u - v) + self.normals[1] * u + self.normals[2] * v)
            .normalize();
        rec.set_face_normal(r, normal);
        rec.u = self.uvs[0].x() * (1.0 - u - v) + self.uvs[1].x() * u + self.uvs[2].x() * v;
        rec.v = self.uvs[0].y() * (1.0 - u - v) + self.uvs[1].y() * u + self.uvs[2].y() * v;
        rec.mat = Rc::clone(&self.mat);

        true
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct TriangleMesh {
    bvh: Bvh,
}

impl TriangleMesh {
    pub fn new(obj_path: &str, position: Vec3, scale: f64, mat: Rc<dyn Material>) -> Self {
        let obj_path = Path::new(obj_path);
        let obj_file = std::fs::read_to_string(obj_path).unwrap();
        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut uvs = Vec::new();
        let mut faces = Vec::new();

        for line in obj_file.lines() {
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("v") => {
                    let x: f64 = parts.next().unwrap().parse().unwrap();
                    let y: f64 = parts.next().unwrap().parse().unwrap();
                    let z: f64 = parts.next().unwrap().parse().unwrap();
                    vertices.push(scale * Point3::new(x, y, z) + position);
                }
                Some("vn") => {
                    let x: f64 = parts.next().unwrap().parse().unwrap();
                    let y: f64 = parts.next().unwrap().parse().unwrap();
                    let z: f64 = parts.next().unwrap().parse().unwrap();
                    normals.push(Vec3::new(x, y, z));
                }
                Some("vt") => {
                    let u: f64 = parts.next().unwrap().parse().unwrap();
                    let v: f64 = parts.next().unwrap().parse().unwrap();
                    uvs.push(Point3::new(u, v, 0.0));
                }
                Some("f") => {
                    let mut _vertices = [Point3::zeros(); 3];
                    let mut _normals = [Vec3::zeros(); 3];
                    let mut _uvs = [Point3::zeros(); 3];
                    for i in 0..3 {
                        let indices: Vec<usize> = parts
                            .next()
                            .unwrap()
                            .split('/')
                            .map(|s| s.parse().unwrap())
                            .collect();
                        _vertices[i] = vertices[indices[0] - 1];
                        _normals[i] = normals[indices[2] - 1];
                        _uvs[i] = uvs[indices[1] - 1];
                    }
                    let face = Triangle::new(_vertices, _normals, _uvs, mat.clone()).into();
                    faces.push(face);
                }
                _ => {}
            }
        }

        let before_bvh = Instant::now();
        let bvh = Bvh::new(&mut faces);
        println!(
            "BVH Construction: {:.2?}ms",
            before_bvh.elapsed().as_millis()
        );

        Self { bvh }
    }
}

impl Hittable for TriangleMesh {
    fn hit(&self, r: &Ray, t: Interval, rec: &mut HitRecord) -> bool {
        self.bvh.hit(r, t, rec)
        // self.list.hit(r, t, rec)
    }

    fn bounding_box(&self) -> &AABB {
        self.bvh.bounding_box()
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}
