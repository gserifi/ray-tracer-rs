use crate::utils::{Point3, Vec3, Vec3Ext};
use rand::rngs::ThreadRng;
use rand::Rng;

#[derive(Debug)]
pub struct Perlin {
    rand_vec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Default for Perlin {
    fn default() -> Self {
        let point_count = 256;
        let mut rng = rand::thread_rng();
        let mut rand_vec = Vec::with_capacity(point_count);
        for _ in 0..point_count {
            rand_vec.push((Vec3::random_uniform_vector(&mut rng) * 2.0 - Vec3::ones()).normalize());
        }

        let perm_x = Self::perlin_generate_perm(&mut rng);
        let perm_y = Self::perlin_generate_perm(&mut rng);
        let perm_z = Self::perlin_generate_perm(&mut rng);

        Self {
            rand_vec,
            perm_x,
            perm_y,
            perm_z,
        }
    }
}

impl Perlin {
    fn perlin_generate_perm(rng: &mut ThreadRng) -> Vec<usize> {
        let mut perm: Vec<usize> = (0..256).collect();
        Self::permute(rng, &mut perm);
        perm
    }

    fn permute(rng: &mut ThreadRng, p: &mut [usize]) {
        for i in (1..p.len()).rev() {
            let target = rng.gen_range(0..=i);
            p.swap(i, target);
        }
    }
}

impl Perlin {
    pub fn turb(&self, p: &Point3, depth: usize) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as isize;
        let j = p.y().floor() as isize;
        let k = p.z().floor() as isize;

        let mut c = [[[Vec3::zeros(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let idi = ((i + di as isize) & 255) as usize;
                    let jdj = ((j + dj as isize) & 255) as usize;
                    let kdk = ((k + dk as isize) & 255) as usize;
                    c[di][dj][dk] =
                        self.rand_vec[self.perm_x[idi] ^ self.perm_y[jdj] ^ self.perm_z[kdk]];
                }
            }
        }

        Self::perlin_interp(&c, u, v, w)
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww))
                        * c[i][j][k].dot(&weight_v);
                }
            }
        }

        accum
    }
}
