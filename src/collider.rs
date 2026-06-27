use crate::Shape;
use macroquad::prelude::*;

pub struct Manifold {
    pub normal: Vec2,
    pub depth: f32,
}

pub struct Collider {
    pub shape: Shape,
    pub offset: Vec2,
}

pub struct Colliders {
    pub solid: Option<Collider>,
    pub interact: Option<Collider>,
    pub attack: Option<Collider>,
}

fn vertices(shape: &Shape, pos: Vec2) -> Vec<Vec2> {
    match shape {
        Shape::Rect(size) => vec![
            pos,
            pos + vec2(size.x, 0.0),
            pos + *size,
            pos + vec2(0.0, size.y),
        ],
        Shape::Polygon(verts) => verts.iter().map(|&v| v + pos).collect(),
        Shape::Disk(_) => panic!("vertices() called on Disk"),
    }
}

fn axes(verts: &[Vec2]) -> impl Iterator<Item = Vec2> + '_ {
    let n = verts.len();
    (0..n).map(move |i| {
        let edge = verts[(i + 1) % n] - verts[i];
        vec2(-edge.y, edge.x).normalize()
    })
}

fn project(verts: &[Vec2], axis: Vec2) -> (f32, f32) {
    verts
        .iter()
        .map(|v| v.dot(axis))
        .fold((f32::INFINITY, f32::NEG_INFINITY), |(lo, hi), d| {
            (lo.min(d), hi.max(d))
        })
}

fn overlap((lo_a, hi_a): (f32, f32), (lo_b, hi_b): (f32, f32)) -> Option<f32> {
    let o = hi_a.min(hi_b) - lo_a.max(lo_b);
    (o > 0.0).then_some(o)
}

fn sat_poly_poly(verts_a: &[Vec2], verts_b: &[Vec2]) -> Option<Manifold> {
    let (normal, depth) = axes(verts_a)
        .chain(axes(verts_b))
        .map(|axis| overlap(project(verts_a, axis), project(verts_b, axis)).map(|d| (axis, d)))
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())?;

    let center_a: Vec2 = verts_a.iter().sum::<Vec2>() / verts_a.len() as f32;
    let center_b: Vec2 = verts_b.iter().sum::<Vec2>() / verts_b.len() as f32;
    let normal = if normal.dot(center_a - center_b) < 0.0 {
        -normal
    } else {
        normal
    };

    Some(Manifold { normal, depth })
}

// Returns a manifold with normal pointing from poly toward disk.
fn sat_disk_poly(disk_pos: Vec2, radius: f32, poly_verts: &[Vec2]) -> Option<Manifold> {
    let closest = poly_verts.iter().min_by(|a, b| {
        a.distance_squared(disk_pos)
            .partial_cmp(&b.distance_squared(disk_pos))
            .unwrap()
    })?;
    let to_disk = disk_pos - *closest;
    let vertex_axis = if to_disk.length_squared() > 0.0 {
        to_disk.normalize()
    } else {
        Vec2::Y
    };

    let (normal, depth) = axes(poly_verts)
        .chain(std::iter::once(vertex_axis))
        .map(|axis| {
            let d = disk_pos.dot(axis);
            overlap((d - radius, d + radius), project(poly_verts, axis)).map(|o| (axis, o))
        })
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .min_by(|(_, d1), (_, d2)| d1.partial_cmp(d2).unwrap())?;

    let center_poly: Vec2 = poly_verts.iter().sum::<Vec2>() / poly_verts.len() as f32;
    let normal = if normal.dot(disk_pos - center_poly) < 0.0 {
        -normal
    } else {
        normal
    };

    Some(Manifold { normal, depth })
}

fn sat_disk_disk(pos_a: Vec2, r_a: f32, pos_b: Vec2, r_b: f32) -> Option<Manifold> {
    let diff = pos_a - pos_b;
    let dist = diff.length();
    let sum_r = r_a + r_b;
    if dist >= sum_r {
        return None;
    }
    let normal = if dist > 0.0 { diff / dist } else { Vec2::Y };
    Some(Manifold {
        normal,
        depth: sum_r - dist,
    })
}

// Separating Axis Theorem (https://dyn4j.org/2010/01/sat/)
pub fn sat(a: (&Collider, Vec2), b: (&Collider, Vec2)) -> Option<Manifold> {
    let pos_a = a.1 + a.0.offset;
    let pos_b = b.1 + b.0.offset;

    match (&a.0.shape, &b.0.shape) {
        (Shape::Disk(r_a), Shape::Disk(r_b)) => sat_disk_disk(pos_a, *r_a, pos_b, *r_b),
        (Shape::Disk(r), _) => {
            // normal from sat_disk_poly points poly(B) → disk(A), which is what we want
            sat_disk_poly(pos_a, *r, &vertices(&b.0.shape, pos_b))
        }
        (_, Shape::Disk(r)) => {
            // sat_disk_poly returns normal pointing poly(A) → disk(B), so we flip
            sat_disk_poly(pos_b, *r, &vertices(&a.0.shape, pos_a)).map(|m| Manifold {
                normal: -m.normal,
                depth: m.depth,
            })
        }
        _ => sat_poly_poly(&vertices(&a.0.shape, pos_a), &vertices(&b.0.shape, pos_b)),
    }
}
