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
        let u = p.x() - f64::floor(p.x());
        let v = p.y() - f64::floor(p.y());
        let w = p.z() - f64::floor(p.z());

        let i = f64::floor(p.x()) as i32;
        let j = f64::floor(p.y()) as i32;
        let k = f64::floor(p.z()) as i32;

        let mut c: [[[f64; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.randfloat[index as usize];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
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

    #[allow(clippy::needless_range_loop)]
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1.0 - i as f64) * (1.0 - u))
                        * (j as f64 * v + (1.0 - j as f64) * (1.0 - v))
                        * (k as f64 * w + (1.0 - k as f64) * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }
        accum
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}
