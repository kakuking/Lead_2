pub use crate::common::*;

pub fn ceil<T, const N: usize>(input: T) -> T 
where
    T: Clone + Into<[Float; N]> + From<[Float; N]>,
{
    let coords: [Float; N] = input.clone().into();
    let ceiled = coords.map(|x| x.ceil());

    T::from(ceiled)
}

pub fn floor<T, const N: usize>(input: T) -> T 
where
    T: Clone + Into<[Float; N]> + From<[Float; N]>,
{
    let coords: [Float; N] = input.clone().into();
    let floored = coords.map(|x| x.floor());

    T::from(floored)
}

pub fn min<T, const N: usize>(a: &T, b: &T) -> T 
where
    T: Clone + Into<[Float; N]> + From<[Float; N]>,
{
    let a_coords: [Float; N] = a.clone().into();
    let b_coords: [Float; N] = b.clone().into();
    let min_coords = std::array::from_fn(|i| a_coords[i].min(b_coords[i]));

    T::from(min_coords)
}

pub fn max<T, const N: usize>(a: &T, b: &T) -> T 
where
    T: Clone + Into<[Float; N]> + From<[Float; N]>,
{
    let a_coords: [Float; N] = a.clone().into();
    let b_coords: [Float; N] = b.clone().into();
    let max_coords = std::array::from_fn(|i| a_coords[i].max(b_coords[i]));

    T::from(max_coords)
}