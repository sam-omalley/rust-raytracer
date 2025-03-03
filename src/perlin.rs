use crate::common;
use crate::vec3::{Point3, Vec3, dot};

const POINT_COUNT: usize = 256;

#[derive(Clone)]
pub struct Perlin {
    randvec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut p = Perlin {
            randvec: [Vec3::zero(); POINT_COUNT],
            perm_x: [0; POINT_COUNT],
            perm_y: [0; POINT_COUNT],
            perm_z: [0; POINT_COUNT],
        };

        for val in p.randvec.iter_mut() {
            *val = Vec3::random_range(-1.0, 1.0);
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

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] = self.randvec[index as usize];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
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
    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;

                    let weight_v = Vec3::new(u - i_f, v - j_f, w - k_f);
                    accum += (i_f * uu + (1.0 - i_f) * (1.0 - u))
                        * (j_f * vv + (1.0 - j_f) * (1.0 - v))
                        * (k_f * ww + (1.0 - k_f) * (1.0 - w))
                        * dot(c[i][j][k], weight_v);
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
