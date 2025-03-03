use crate::common;
use crate::vec3::Point3;

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    randfloat: [f64; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut p = Perlin {
            randfloat: [0.0; POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        };

        for val in p.randfloat.iter_mut() {
            *val = common::random_double();
        }

        Self::perlin_generate_perm(&mut p.perm_x);
        Self::perlin_generate_perm(&mut p.perm_y);
        Self::perlin_generate_perm(&mut p.perm_z);

        p
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let i = ((4.0 * p.x()) as i32) & 255;
        let j = ((4.0 * p.y()) as i32) & 255;
        let k = ((4.0 * p.z()) as i32) & 255;

        let index: i32 =
            self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize];

        self.randfloat[index as usize]
    }

    fn perlin_generate_perm(p: &mut [i32; POINT_COUNT]) {
        for (counter, val) in p.iter_mut().enumerate() {
            *val = counter as i32;
        }

        Self::permute(p);
    }

    #[allow(clippy::manual_swap)]
    fn permute(p: &mut [i32; POINT_COUNT]) {
        for i in (1..POINT_COUNT).rev() {
            let target = common::random_int(0, i as i32) as usize;
            let tmp = p[i];
            p[i] = p[target];
            p[target] = tmp;
        }
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
