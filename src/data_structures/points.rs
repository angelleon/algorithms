use jsonable::JSONAble;
pub use point::{println_Point2D, Dist, Point, Point2D, PointComp};

pub struct NPoint<T: Default + Dist> {
    pub p: T,
    pub clasif: usize,
}

impl<T: Default + Dist> NPoint<T> {
    pub fn new() -> NPoint<T> {
        NPoint {
            p: Default::default(),
            clasif: 0usize,
        }
    }
    pub fn new_from(p: T, clasif: usize) -> NPoint<T> {
        NPoint { p, clasif }
    }
}

impl<T: Default + Dist + JSONAble<T>> JSONAble<T> for NPoint<T> {
    fn to_json_string(&self) -> String {
        let mut s = String::from("{\"p\":");
        s.push_str(self.p.to_json_string().as_str());
        s.push_str(",\"clasif\":");
        s.push_str(self.clasif.to_string().as_str());
        s.push_str("}");
        s
    }
}

pub struct KPoint<T: Default + Dist> {
    pub p: T,
    pub count: usize,
}

impl<T: Default + Dist> KPoint<T> {
    pub fn new() -> Self {
        KPoint {
            p: Default::default(),
            count: 0usize,
        }
    }
    pub fn new_from(p: T, count: usize) -> Self {
        KPoint { p, count }
    }
}

impl<T: Default + Dist + JSONAble<T>> JSONAble<T> for KPoint<T> {
    fn to_json_string(&self) -> String {
        let mut s = String::from("{\"p\":");
        s.push_str(self.p.to_json_string().as_str());
        s.push_str(",\"count\":");
        s.push_str(self.count.to_string().as_str());
        s.push_str("}");
        s
    }
}
