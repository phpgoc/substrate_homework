pub enum TrafficList{
    Red,
    Yellow,
    Green
}
pub trait Time {
    fn time(self) -> u32;
}
impl Time for TrafficList{
    fn time(self) -> u32{
        use TrafficList::*;
        match self{
            Red => 50,
            Yellow => 4,
            Green => 46
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn do_test() {
        let yellow = TrafficList::Yellow;
        println!("{}",yellow.time());
    }
}