pub trait Roll {
    type Output;
    fn roll(&self) -> Self::Output;
}