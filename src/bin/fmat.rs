use fmat::{Mat};

fn main() {
	let mut m : Mat<i32> =Mat::new(3, 3);
    for i in 0..3 {
    	m[i][i] = 1;
    }
    println!("matrix:\n{}", m);
    let add1 = Mat::with_vec(vec![vec![1,1,1],vec![1,1,1],vec![1,1,1]]);
    println!("add:\n{}", add1);
    let mut add = m + add1;
    println!("add:\n{}", add);

    let mul1 = Mat::with_vec(vec![vec![1,2,3], 
    	                          vec![2,3,4], 
    	                          vec![3,4,5], 
    	                          vec![4,5,6]]);
    println!("mul1:\n{}", mul1);
    let mul2 = Mat::with_vec(vec![vec![2,3], 
    	                          vec![3,4], 
    	                          vec![4,5]]);
    println!("mul2:\n{}", mul2);
    let mul3 = mul1 * mul2;
    println!("mul:\n{}", mul3);
}