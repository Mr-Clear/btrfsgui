
pub trait ResultExt<T, E> {
    fn contains2(&self, t: &T) -> bool;
}

impl<T: PartialEq<T>, E> ResultExt<T, E> for Result<T, E> {
    fn contains2(&self, t: &T) -> bool {
        match self {
            Ok(v) => t == v,
            Err(_) => false,
        }
    }
}
