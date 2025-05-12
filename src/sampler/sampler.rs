use crate::common::*;

pub trait Sampler: Debug + Clone {
    fn array_1d_offset(&self) -> usize;
    fn array_2d_offset(&self) -> usize;
    fn current_pixel(&self) -> Point2;
    fn current_pixel_sample_idx(&self) -> usize;
    fn samples_1d_array_sizes(&mut self) -> &mut Vec<usize>;
    fn samples_2d_array_sizes(&mut self) -> &mut Vec<usize>;
    fn sample_array_1d(&mut self) -> &mut Vec<Vec<Float>>;
    fn sample_array_2d(&mut self) -> &mut Vec<Vec<Point2>>;
    fn samples_per_pixel(&self) -> usize;

    fn set_array_1d_offset(&mut self, other: usize);
    fn set_array_2d_offset(&mut self, other: usize);
    fn set_current_pixel(&mut self, other: Point2);
    fn set_current_pixel_sample_idx(&mut self, other: usize);
    fn set_samples_per_pixel(&mut self, other: usize);
    fn start_next_sample(&mut self) -> bool;
    fn set_sample_number(&mut self, sample_num: usize) -> bool;

    fn start_pixel(&mut self, p: Point2) {
        self.set_current_pixel(p);
        self.set_current_pixel_sample_idx(0usize);
        self.set_array_1d_offset(0usize);
        self.set_array_2d_offset(0usize);
    }
    fn get_camera_sample(&mut self, p_raster: &Point2) -> CameraSample {
        let p_film = p_raster.coords + self.get_2d().coords;
        let time = self.get_1d();
        let p_lens = self.get_2d();

        CameraSample {
            p_film: Point2::new(p_film.x, p_film.y),
            time,
            p_lens
        }
    }
    fn request_1d_array(&mut self, n: usize) {
        self.samples_1d_array_sizes().push(n);

        let new_samples = vec![0.0; n * self.samples_per_pixel()];
        self.sample_array_1d().push(new_samples);
    }
    fn request_2d_array(&mut self, n: usize) {
        self.samples_2d_array_sizes().push(n);

        let new_samples = vec![Point2::new(0.0, 0.0); n * self.samples_per_pixel()];
        self.sample_array_2d().push(new_samples);
    }
    fn round_count(&self, n: usize) -> usize {
        n
    }
    fn get_1d_array(&mut self, n: usize) -> Option<&[Float]> {
        if self.array_1d_offset() == self.sample_array_1d().len() {
            return None;
        }
        let array_1d_offset = self.array_1d_offset();
        self.set_array_1d_offset(self.array_1d_offset() + 1);

        let idx = self.current_pixel_sample_idx() * n;
        let array = &self.sample_array_1d()[array_1d_offset - 1];
        
        array.get(idx..idx+n)
    }
    fn get_2d_array(&mut self, n: usize) -> Option<&[Point2]> {
        if self.array_1d_offset() == self.sample_array_1d().len() {
            return None;
        }
        let array_2d_offset = self.array_2d_offset();
        self.set_array_2d_offset(self.array_2d_offset() + 1);

        let idx = self.current_pixel_sample_idx() * n;
        let array = &self.sample_array_2d()[array_2d_offset - 1];
        
        array.get(idx..idx+n)
    }
    fn default_start_next_sample(&mut self) -> bool{
        self.set_array_1d_offset(0usize);
        self.set_array_2d_offset(0usize);

        self.set_current_pixel_sample_idx(self.current_pixel_sample_idx() + 1);

        self.current_pixel_sample_idx() < self.samples_per_pixel()
    }
    fn default_set_sample_number(&mut self, sample_num: usize) -> bool {
        self.set_array_1d_offset(0usize);
        self.set_array_2d_offset(0usize);

        self.set_current_pixel_sample_idx(sample_num);

        self.current_pixel_sample_idx() < self.samples_per_pixel()
    }
    fn get_1d(&mut self) -> Float;
    fn get_2d(&mut self) -> Point2;
    // fn clone(&self, seed: usize) -> Arc<Self>;
}