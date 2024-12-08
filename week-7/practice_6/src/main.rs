fn main() {
    
    // Create two vector
    let v = vec![1,2,3,4,5,6,7,8];
    let x = vec![5,6,7,8,9,10,11];

    // Use a for loop with zip to iterate over both vectors
    for (a,b) in v.iter().zip(x.iter()) {
        println! ("{}", index);
        let sum = v[index] + x[index];
        println!("{:?}",sum);
    }
}
