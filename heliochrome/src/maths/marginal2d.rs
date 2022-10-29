// adapted from https://github.com/rgl-epfl/brdf-loader

use std::{default, error::Error, ops::Mul, slice, io::Write};

use rayon::vec;

use super::vec2;

#[derive(Debug, Clone)]
pub struct Marginal2d<const ArraySize: usize> {
    /// Resolution of the discretized density function
    size: vec2,

    /// Size of a bilinear patch in the unit square
    patch_size: vec2,
    inv_patch_size: vec2,

    /// Resolution of each parameter (optional)
    param_size: [usize; ArraySize],

    /// Stride per parameter in units of sizeof(float)
    param_strides: [usize; ArraySize],

    /// Discretization of each parameter domain
    param_values: [Vec<f32>; ArraySize],

    /// Density values
    data: Vec<f32>,

    /// Marginal and conditional PDFs
    marginal_cdf: Vec<f32>,
    conditional_cdf: Vec<f32>,
}

fn find_interval(sized: usize, f: &dyn Fn(usize) -> bool) -> usize {
    let mut size = sized as isize - 2;
    let mut first = 1;
    while size > 0 {
        let half = size >> 1;
        let middle = first + half;
        let pr = f(middle as usize);

        first = if pr { middle + 1 } else { first };
        size = if pr { size - (half + 1) } else { half };
    }

    (first - 1).clamp(0, sized as isize - 2) as usize
}

impl<const Dimension: usize> Marginal2d<Dimension>
where
    [(); 2 * Dimension]:,
{
    pub fn new(
        size: &vec2,
        data: &[f32],
        // param_res: [u32; Dimension],
        param_values: [Vec<f32>; Dimension],
        normalize: bool,
        build_cdf: bool,
    ) -> Result<Self, Box<dyn Error>> {
        if build_cdf && !normalize {
            return Err("if `build_cdf` is true, `normalize` must also be true")?;
        }
        let mut slices = 1usize;
        let mut param_size = [0; Dimension];
        let mut param_strides = [0; Dimension];
        for i in (0..Dimension).rev() {
            // FIXME: was reversed before
            if param_values[i].is_empty() {
                return Err("parameter resolution not be empty")?;
            }
            param_size[i] = param_values[i].len();
            param_strides[i] = if param_size[i] > 1 { slices } else { 0 };
            slices *= param_size[i];
        }

        let inv_patch_size = *size - vec2::splat(1.0);

        let n_values = size.prod() as usize;
        let mut data_out = vec![0.0; n_values * slices];
        let mut data_out_off = 0;
        let mut data_off = 0;

        let (marginal_cdf, conditional_cdf) = if build_cdf {
            let mut marginal_cdf = vec![0.0; slices * size.y as usize];
            let mut conditional_cdf = vec![0.0; slices * n_values];

            let mut marginal_off = 0;
            let mut conditional_off = 0;

            for slice in 0..slices {
                for y in 0..size.y as usize {
                    let mut sum = 0.0;
                    let mut i = y * size.x as usize;
                    for x in 0..(size.x as usize - 1) {
                        sum += 0.5 * (data[data_off + i] as f64 + data[data_off + i + 1] as f64);
                        conditional_cdf[conditional_off + i + 1] = sum as f32;
                        i += 1;
                    }
                }

                marginal_cdf[marginal_off + 0] = 0.0;
                let mut sum = 0.0;
                for y in 0..(size.y as usize - 1) {
                    sum += 0.5
                        * (conditional_cdf[conditional_off + (y + 1) * size.x as usize - 1] as f64
                            + conditional_cdf[conditional_off + (y + 2) * size.x as usize - 1]
                                as f64);
                    marginal_cdf[marginal_off + y + 1] = sum as f32;
                }

                let normalization = 1.0 / marginal_cdf[marginal_off + size.y as usize - 1];
                for i in 0..n_values {
                    conditional_cdf[conditional_off + i] *= normalization;
                }
                for i in 0..size.y as usize {
                    marginal_cdf[marginal_off + i] *= normalization;
                }
                for i in 0..n_values {
                    data_out[data_out_off + i] = data[data_off + i] * normalization;
                }

                marginal_off += size.y as usize;
                conditional_off += n_values;
                data_out_off += n_values;
                data_off += n_values;
            }

            (marginal_cdf, conditional_cdf)
        } else {
            data_out.clone_from_slice(data);

            for slice in 0..slices {
                let mut normalization = 1.0 / inv_patch_size.prod();
                if normalize {
                    let mut sum = 0.0;
                    for y in 0..size.y as usize - 1 {
                        let mut i = y * size.x as usize;
                        for x in 0..(size.x as usize - 1) {
                            let v00 = data[data_off + i];
                            let v10 = data[data_off + i + 1];
                            let v01 = data[data_off + i + size.x as usize];
                            let v11 = data[data_off + i + 1 + size.x as usize];
                            let avg = 0.25 * (v00 + v10 + v01 + v11);
                            sum += avg as f64;
                            i += 1;
                        }
                    }
                    normalization = 1.0 / sum as f32;
                }
                for k in 0..n_values {
                    data_out[data_out_off + k] = data[data_off + k] * normalization;
                }

                data_out_off += n_values;
                data_off += n_values;
            }

            (vec![], vec![])
        };

        Ok(Self {
            size: *size,
            patch_size: vec2::splat(1.0) / (*size - vec2::splat(1.0)),
            inv_patch_size,
            param_size,
            param_values,
            param_strides,
            data: data_out,
            marginal_cdf,
            conditional_cdf,
        })
    }

    fn lookup(&self, d: usize, data: &[f32], i0: usize, size: usize, param_weight: &[f32]) -> f32 {
        if d != 0 {
            let i1 = i0 + self.param_strides[d - 1] * size;
            let w0 = param_weight[2 * d - 2];
            let w1 = param_weight[2 * d - 1];
            let v0 = self.lookup(d - 1, data, i0, size, param_weight);
            let v1 = self.lookup(d - 1, data, i1, size, param_weight);
            v0.mul_add(w0, v1 * w1)
        } else {
            data[i0]
        }
    }

    pub fn sample(&self, mut sample: vec2, param: &[f32]) -> (vec2, f32) {
        // Avoid degeneracies at the extrema
        sample = sample.clamp(1.0 - 0.999999940395355225, 0.999999940395355225);

        let mut param_weight = [0.0; 2 * Dimension];
        let mut slice_offset = 0;
        for d in 0..Dimension {
            if self.param_size[d] == 1 {
                param_weight[2 * d] = 1.0;
                param_weight[2 * d + 1] = 0.0;
                continue;
            }

            let param_index = find_interval(self.param_size[d], &|idx| {
                self.param_values[d][idx] <= param[d]
            });

            let p0 = self.param_values[d][param_index];
            let p1 = self.param_values[d][param_index + 1];

            param_weight[2 * d + 1] = ((param[d] - p0) / (p1 - p0)).clamp(0.0, 1.0);
            param_weight[2 * d] = 1.0 - param_weight[2 * d + 1];
            slice_offset += self.param_strides[d] * param_index;
        }
        let mut offset = if Dimension == 0 {
            0
        } else {
            slice_offset * self.size.y as usize
        };

        let fetch_marginal = |idx: usize| {
            self.lookup(
                Dimension,
                &self.marginal_cdf,
                offset + idx,
                self.size.y as usize,
                &param_weight,
            )
        };

        let row = find_interval(self.size.y as usize, &|idx| fetch_marginal(idx) < sample.y);
        sample.y -= fetch_marginal(row);

        let slice_size = self.size.prod() as usize;
        offset = row * self.size.x as usize;

        if Dimension != 0 {
            offset += slice_offset * slice_size
        }

        let r0 = self.lookup(
            Dimension,
            &self.conditional_cdf,
            offset + self.size.x as usize - 1,
            slice_size,
            &param_weight,
        );

        let r1 = self.lookup(
            Dimension,
            &self.conditional_cdf,
            offset + self.size.x as usize * 2 - 1,
            slice_size,
            &param_weight,
        );
        
        let is_const = (r0 - r1).abs() < 0.0001 * (r0 + r1);
        sample.y = if is_const {
            (2.0 * sample.y) / (r0 + r1)
        } else {
            (r0 - (r0 * r0 - 2.0 * sample.y * (r0 - r1)).sqrt()) / (r0 - r1)
        };

        sample.x *= (1.0 - sample.y) * r0 + sample.y * r1;

        let fetch_conditional = |idx| {
            let v0 = self.lookup(
                Dimension,
                &self.conditional_cdf,
                offset + idx,
                slice_size,
                &param_weight,
            );
            let v1 = self.lookup(
                Dimension,
                &self.conditional_cdf[self.size.x as usize..],
                offset + idx,
                slice_size,
                &param_weight,
            );

            (1.0 - sample.y) * v0 + sample.y * v1
        };

        let col = find_interval(self.size.x as usize, &|idx| {
            fetch_conditional(idx) < sample.x
        });

        sample.x -= fetch_conditional(col);

        offset += col;


        let v00 = self.lookup(Dimension, &self.data, offset, slice_size, &param_weight);
        let v10 = self.lookup(
            Dimension,
            &self.data[1..],
            offset,
            slice_size,
            &param_weight,
        );
        let v01 = self.lookup(
            Dimension,
            &self.data[self.size.x as usize..],
            offset,
            slice_size,
            &param_weight,
        );
        let v11 = self.lookup(
            Dimension,
            &self.data[self.size.x as usize + 1..],
            offset,
            slice_size,
            &param_weight,
        );
        let c0 = (1.0 - sample.y).mul_add(v00, sample.y * v01);
        let c1 = (1.0 - sample.y).mul_add(v10, sample.y * v11);
        let is_const = (c0 - c1).abs() < 1e-4 * (c0 + c1);
        sample.x = if is_const {
            2.0 * sample.x / (c0 + c1)
        } else {
            (c0 - (c0 * c0 - 2.0 * sample.x * (c0 - c1)).sqrt()) / (c0 - c1)
        };
        
        (
            (vec2::new(col as f32, row as f32) + sample) * self.patch_size,
            ((1.0 - sample.x) * c0 + sample.x * c1) * self.inv_patch_size.prod(),
        )
    }

    pub fn eval(&self, mut pos: vec2, param: &[f32]) -> f32 {
        let mut param_weight = [0.0; 2 * Dimension];
        let mut slice_offset = 0;

        for d in 0..Dimension {
            if self.param_size[d] == 1 {
                param_weight[2 * d] = 1.0;
                param_weight[2 * d + 1] = 0.0;
                continue;
            }

            let param_index = find_interval(self.param_size[d], &|idx| {
                self.param_values[d][idx] <= param[d]
            });

            let p0 = self.param_values[d][param_index];
            let p1 = self.param_values[d][param_index + 1];

            param_weight[2 * d + 1] = ((param[d] - p0) / (p1 - p0)).clamp(0.0, 1.0);
            param_weight[2 * d] = 1.0 - param_weight[2 * d + 1];
            slice_offset += self.param_strides[d] * param_index;
        }

        pos *= self.inv_patch_size;
        let offset = pos.min(&(self.size - vec2::splat(2.0))).floor();

        let w1 = pos - offset;
        let w0 = vec2::splat(1.0) - w1;

        let size = self.size.prod() as usize;
        let index = (offset.x + offset.y * self.size.x) as usize
            + if Dimension != 0 {
                slice_offset * size
            } else {
                0
            };

        let v00 = self.lookup(Dimension, &self.data, index, size, &param_weight);
        let v10 = self.lookup(Dimension, &self.data[1..], index, size, &param_weight);
        let v01 = self.lookup(
            Dimension,
            &self.data[self.size.x as usize..],
            index,
            size,
            &param_weight,
        );
        let v11 = self.lookup(
            Dimension,
            &self.data[self.size.x as usize + 1..],
            index,
            size,
            &param_weight,
        );

        return w0.y.mul_add(
            w0.x.mul_add(v00, w1.x * v10),
            w1.y * w0.x.mul_add(v01, w1.x * v11),
        ) * self.inv_patch_size.prod();
    }
}

pub type Warp2D0 = Marginal2d<0>;
pub type Warp2D2 = Marginal2d<2>;
pub type Warp2D3 = Marginal2d<3>;
