
fn calculate_sum(number: &[u32])-> Option<u32>{
    let mut sum = 0u32;
    
    for &num in number {
        match sum.checked_add(num){
            Some(result)=> sum = result,
            None => return None,
        }
    }
    
    Some(sum)
}


fn main(){
    let numbers = vec![1,2,3,4,5,6,7,8,9];

    match calculate_sum(&numbers){
        Some(result)=> println!("result:{}",result),
        None => println!("None"),
    }
}
