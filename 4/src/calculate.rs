fn do_calculate(arr: &[u32]) -> Option<u32> {
    let max_u32 = u32::MAX as u64;
    let mut sum = 0;
    for &i in arr.iter() {
        sum += i as u64;
        if sum > max_u32{
            return None;
        }
    }
    Some(sum as u32)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn do_test_calculate() {
        let input = [1,2,3];
        println!("{:?}",do_calculate(&input));
        let input = [u32::MAX,2,3];
        println!("{:?}",do_calculate(&input));
    }
    
}