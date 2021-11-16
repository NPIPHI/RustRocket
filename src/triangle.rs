use nalgebra_glm::Vec3;

pub struct Triangle {
    pub p1: Vec3,
    pub p2: Vec3,
    pub p3: Vec3
}

impl Triangle {
    pub fn new(p1: Vec3, p2: Vec3, p3: Vec3) -> Triangle {
        return Triangle{p1, p2, p3};
    }

    pub fn normal(&self) -> Vec3 {
        let v1 = self.p2 - self.p1;
        let v2 = self.p3 - self.p2;
        return v1.cross(&v2).normalize();
    }

    pub fn to_array(&self) -> ([f32; 9], [f32; 9]) {
        let verts: [f32; 9] = [self.p1.x, self.p1.y, self.p1.z, self.p2.x, self.p2.y, self.p2.z, self.p3.x, self.p3.y, self.p3.z];
        let normal = self.normal();
        let normals: [f32; 9] = [normal.x, normal.y, normal.z, normal.x, normal.y, normal.z, normal.x, normal.y, normal.z];
        return (verts, normals);
    }
}