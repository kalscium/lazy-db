pub trait CustomLazyType {
    fn hollow() -> Self where Self: Sized;
}