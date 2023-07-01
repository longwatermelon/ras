use glam::Vec3;

pub fn barycentric(p: Vec3, tri: &[Vec3; 3]) -> Vec3 {
    let v0: Vec3 = tri[1] - tri[0];
    let v1: Vec3 = tri[2] - tri[0];
    let v2: Vec3 = p - tri[0];
    let d00: f32 = v0.dot(v0);
    let d01: f32 = v0.dot(v1);
    let d11: f32 = v1.dot(v1);
    let d20: f32 = v2.dot(v0);
    let d21: f32 = v2.dot(v1);
    let denom: f32 = d00 * d11 - d01 * d01;

    let mut bary: Vec3 = Vec3::ZERO;
    bary.y = (d11 * d20 - d01 * d21) / denom;
    bary.z = (d00 * d21 - d01 * d20) / denom;
    bary.x = 1.0 - bary.y - bary.z;

    bary
}

