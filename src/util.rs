use glam::Vec3;

pub struct BaryCache {
    v0: Vec3,
    v1: Vec3,
    d00: f32,
    d01: f32,
    d11: f32,
    oodenom: f32
}

impl BaryCache {
    pub fn new(tri: &[Vec3; 3]) -> Self {
        let v0: Vec3 = tri[1] - tri[0];
        let v1: Vec3 = tri[2] - tri[0];

        let d00: f32 = v0.dot(v0);
        let d01: f32 = v0.dot(v1);
        let d11: f32 = v1.dot(v1);


        let denom: f32 = d00 * d11 - d01 * d01;

        Self {
            v0, v1, d00, d01, d11,
            oodenom: 1. / denom
        }
    }
}

pub fn barycentric(p: Vec3, tri: &[Vec3; 3], cache: &BaryCache) -> Vec3 {
    let v2: Vec3 = p - tri[0];
    let d20: f32 = v2.dot(cache.v0);
    let d21: f32 = v2.dot(cache.v1);

    let mut bary: Vec3 = Vec3::ZERO;
    bary.y = (cache.d11 * d20 - cache.d01 * d21) * cache.oodenom;
    bary.z = (cache.d00 * d21 - cache.d01 * d20) * cache.oodenom;
    bary.x = 1.0 - bary.y - bary.z;

    bary
}

