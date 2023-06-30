use glam::Vec3;

#[derive(Clone, Copy)]
pub struct Vertex {
    pub pos: Vec3
}

impl Vertex {
    pub fn new(pos: Vec3) -> Self {
        Self { pos }
    }
}

pub struct Screen {
    pub w: usize,
    pub h: usize,
    pub color: Vec<Vec3>,
    pub zbuf: Vec<f32>
}

impl Screen {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w, h,
            color: vec![Vec3::new(0., 0., 0.); w * h],
            zbuf: vec![f32::INFINITY; w * h]
        }
    }

    pub fn clear(&mut self) {
        self.color = vec![Vec3::new(0., 0., 0.); self.w * self.h];
        self.zbuf = vec![f32::INFINITY; self.w * self.h];
    }
}

struct MovingPoint {
    orig: Vertex,
    dxdy: f32,
    dzdy: f32
}

impl MovingPoint {
    fn new(orig: Vertex, to: Vertex) -> Self {
        Self {
            orig,
            dxdy: (to.pos.x - orig.pos.x) / (to.pos.y - orig.pos.y),
            dzdy: (to.pos.z - orig.pos.z) / (to.pos.y - orig.pos.y)
        }
    }

    fn advance_dy(&mut self, dy: f32) {
        self.orig.pos.x += self.dxdy * dy;
        self.orig.pos.y += dy;
        self.orig.pos.z += self.dzdy * dy;
    }
}

pub fn tri(verts: &[Vertex; 3], scr: &mut Screen) {
    let mut scrverts: [Vertex; 3] = verts.map(|v| project_vert(v, scr.w, scr.h));
    // Sort scrverts by y, with [0] being lowest and [2] being highest
    scrverts.sort_by(|a, b| a.pos.y.partial_cmp(&b.pos.y).unwrap());

    fill_tri(&scrverts, scr);
}

pub fn project_vert(v: Vertex, w: usize, h: usize) -> Vertex {
    Vertex::new(Vec3::new(
        (v.pos.x / v.pos.z + 0.5) * w as f32,
        (v.pos.y / v.pos.z + 0.5) * h as f32,
        v.pos.z
    ))
}

fn fill_tri(scrverts: &[Vertex; 3], scr: &mut Screen) {
    let mut mp01: MovingPoint = MovingPoint::new(scrverts[0], scrverts[1]);
    let mut mp02: MovingPoint = MovingPoint::new(scrverts[0], scrverts[2]);
    let mut mp12: MovingPoint = MovingPoint::new(scrverts[1], scrverts[2]);

    let (left0, right0) = if mp01.dxdy < mp02.dxdy {
        (&mut mp01, &mut mp02)
    } else {
        (&mut mp02, &mut mp01)
    };

    fill_tri_part(
        scrverts[0].pos.y, scrverts[1].pos.y,
        left0, right0, scr
    );

    let (left1, right1) = if mp02.orig.pos.x < mp12.orig.pos.x {
        (&mut mp02, &mut mp12)
    } else {
        (&mut mp12, &mut mp02)
    };

    fill_tri_part(
        scrverts[1].pos.y, scrverts[2].pos.y,
        left1, right1, scr
    );
}

fn fill_tri_part(y0: f32, y1: f32,
                 left: &mut MovingPoint, right: &mut MovingPoint,
                 scr: &mut Screen)
{
    let color_slice = scr.color.as_mut_slice();
    let zbuf_slice = scr.zbuf.as_mut_slice();

    for y in (y0 as i32)..(y1 as i32) {
        left.advance_dy(1.);
        right.advance_dy(1.);

        let dzdx: f32 = (right.orig.pos.z - left.orig.pos.z) / (right.orig.pos.x - left.orig.pos.x);
        for x in (left.orig.pos.x as i32)..(right.orig.pos.x as i32) {
            let dx: f32 = x as f32 - left.orig.pos.x;
            let z: f32 = left.orig.pos.z + dzdx * dx;

            let index: usize = (y * scr.w as i32 + x) as usize;
            color_slice[index] = Vec3::new(1., 1., 1.);
            zbuf_slice[index] = z;
        }
    }
}

