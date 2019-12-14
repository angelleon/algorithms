use crate::data_structures::points::{Dist, KPoint, NPoint, Point};

pub type VKPoint<P: Dist + Default> = Vec<KPoint<P>>;
pub type VNPoint<P: Dist + Default> = Vec<NPoint<P>>;

pub struct Model<P: Dist + Default> {
    classes: VKPoint<P>,
    members: VNPoint<P>,
}

impl<P> Model<P>
where
    P: Dist + Default {
    pub fn new() {}
    pub fn train(&mut self, member: NPoint<P>, classIndex: usize) {
        self.members.push(member);
    }
}

fn eucl_dist(x_vals: &Vec<i32>, y_vals: &Vec<i32>) -> f64 {
    assert_eq!(x_vals.len(), y_vals.len());
    let mut sum: i32 = 0;
    let mut i = 0usize;
    while i < x_vals.len() {
        sum += (x_vals[i] - y_vals[i]).pow(2);
        i += 1;
    }
    f64::sqrt(sum as f64)
}

fn manha_dist(x_vals: &Vec<i32>, y_vals: &Vec<i32>) -> f64 {
    assert_eq!(x_vals.len(), y_vals.len());
    let mut sum: i32 = 0;
    let mut i = 0usize;
    while i < x_vals.len() {
        sum += (x_vals[i] - y_vals[i]).abs();
        i += 1;
    }
    sum as f64
}

fn minkow_dist(x_vals: &Vec<i32>, y_vals: &Vec<i32>, q: u32) -> f64 {
    assert_eq!(x_vals.len(), y_vals.len());
    let mut sum: i32 = 0;
    let mut i = 0usize;
    while i < x_vals.len() {
        sum += (x_vals[i] - y_vals[i]).abs().pow(q);
        i += 1;
    }
    (sum as f64).powf(1f64 / q as f64)
}

pub fn knn<P>(model: Model<P>)
where
    P: Dist + Default,
{

}
