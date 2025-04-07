

use crate::common::*;

pub trait Medium: Debug {

}

// impl Medium {
//     pub fn new() -> Self {
//         Medium{}
//     }
// }

#[derive(Debug, Clone)]
pub struct MediumInterface {
    pub inside: Option<Arc<dyn Medium>>,
    pub outside: Option<Arc<dyn Medium>>,
}

impl MediumInterface {
    pub fn new() -> Self {
        Self {
            inside: None,
            outside: None
        }
    }

    pub fn init(inside: Arc<dyn Medium>, outside: Arc<dyn Medium>) -> Self {
        Self {
            inside: Some(inside),
            outside: Some(outside)
        }
    }

    pub fn init_one(medium: Arc<dyn Medium>) -> Self {
        Self {
            inside: Some(medium.clone()),
            outside: Some(medium)
        }
    }

    pub fn inside_outside_same(&self) -> bool {
        let is_inside_some = self.inside.is_some();
        let is_outside_some = self.outside.is_some();

        if !(is_outside_some && is_inside_some) {
            return false;
        }

        if let Some(inside) = &self.inside {
            if let Some(outside) = &self.outside {
                return Arc::ptr_eq(inside, outside);
            }
        }

        return false;
    }

    pub fn is_medium_transition(&self) -> bool {
        !self.inside_outside_same()
    }
}