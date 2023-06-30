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
                self.advance_x(dy),
                self.orig.pos.y + dy,
                self.advance_z(dy)
            )
        )
    }

    fn advance_x(&self, dy: f32) -> f32 {
        self.orig.pos.x + self.dxdy * dy
    }

    fn advance_z(&self, dy: f32) -> f32 {
        self.orig.pos.z + self.dzdy * dy
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
    let mp02: MovingPoint = MovingPoint::new(scrverts[0], scrverts[2]);
    let mp12: MovingPoint = MovingPoint::new(scrverts[1], scrverts[2]);

    let middle_right_of_0: bool = scrverts[1].pos.x > scrverts[0].pos.x;
    let middle_right_of_2: bool = scrverts[1].pos.x > scrverts[2].pos.x;

    fill_tri_part(scrverts[0].pos.y, scrverts[1].pos.y,
        if middle_right_of_0 { &mp02 } else { &mp01 },
        if middle_right_of_0 { &mp01 } else { &mp02 },
        scr
    );

    fill_tri_part(scrverts[1].pos.y, scrverts[2].pos.y,
        if middle_right_of_2 { &mp02 } else { &mp12 },
        if middle_right_of_2 { &mp12 } else { &mp02 },
        scr
    );
}

fn fill_tri_part(y0: f32, y1: f32,
                 left: &MovingPoint, right: &MovingPoint,
                 scr: &mut Screen)
{
    for y in (y0 as i32)..(y1 as i32) {
        let dy: f32 = y as f32 - y0;
        let l: Vertex = left.advance_dy(dy);
        let r: Vertex = right.advance_dy(dy);

        let dzdx: f32 = (r.pos.z - l.pos.z) / (r.pos.x - l.pos.x);
        for x in (l.pos.x as i32)..(r.pos.x as i32) {
            let dx: f32 = x as f32 - l.pos.x;
            let z: f32 = l.pos.z + dzdx * dx;

            let index: usize = scr.index(y as usize, x as usize);
            scr.color[index] = Vec3::new(1., 1., 1.);
            scr.zbuf[index] = z;
        }
    }
}

