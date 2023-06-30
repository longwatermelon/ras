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
            zbuf: vec![0.; w * h]
        }
    }

    pub fn index(&self, mut row: usize, mut col: usize) -> usize {
        row = usize::clamp(row, 0, self.w - 1);
        col = usize::clamp(col, 0, self.h - 1);
        row * self.w + col
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

    fn advance_dy(&self, dy: f32) -> Vertex {
        Vertex::new(
            Vec3::new(
                self.orig.pos.x + self.dxdy * dy,
                self.orig.pos.y + dy,
                self.orig.pos.z + self.dzdy * dy
            )
        )
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
    let mp01: MovingPoint = MovingPoint::new(scrverts[0], scrverts[1]);
    let mut mp02: MovingPoint = MovingPoint::new(scrverts[0], scrverts[2]);
    let mp12: MovingPoint = MovingPoint::new(scrverts[1], scrverts[2]);

    fill_tri_part(
        scrverts[0].pos.y, scrverts[1].pos.y,
        &mp01, &mp02, scr
    );

    mp02.orig = mp02.advance_dy(scrverts[1].pos.y - scrverts[0].pos.y);

    fill_tri_part(
        scrverts[1].pos.y, scrverts[2].pos.y,
        &mp02, &mp12, scr
    );
}

fn fill_tri_part(y0: f32, y1: f32,
                 mp0: &MovingPoint, mp1: &MovingPoint,
                 scr: &mut Screen)
{
    let zero_left_of_one: bool = mp0.advance_dy(0.1).pos.x < mp1.advance_dy(0.1).pos.x;
    let left: &MovingPoint = if zero_left_of_one { mp0 } else { mp1 };
    let right: &MovingPoint = if zero_left_of_one { mp1 } else { mp0 };

    let color_slice = scr.color.as_mut_slice();
    let zbuf_slice = scr.zbuf.as_mut_slice();

    for y in (y0 as i32)..(y1 as i32) {
        let dy: f32 = y as f32 - y0;
        let l: Vertex = left.advance_dy(dy);
        let r: Vertex = right.advance_dy(dy);

        let dzdx: f32 = (r.pos.z - l.pos.z) / (r.pos.x - l.pos.x);
        for x in (l.pos.x as i32)..(r.pos.x as i32) {
            let dx: f32 = x as f32 - l.pos.x;
            let z: f32 = l.pos.z + dzdx * dx;

            let index: usize = (y * scr.w as i32 + x) as usize;
            color_slice[index] = Vec3::new(1., 1., 1.);
            zbuf_slice[index] = z;
        }
    }
}

