pub trait Particle<T> {
    fn reverse(self) -> T;
    fn init(self)-> T;
    fn value(self) -> i32;
}
