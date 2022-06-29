
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::{Add, Sub, Mul, Index, IndexMut, Deref};
use std::marker::Copy;
use std::iter::Sum;
//use std::convert::Into;

#[derive(Debug)]
pub struct Mat <T> where T: Default{
    m: usize, n: usize,
    vec: Vec<T>
}

pub struct Scal<T> {
    v: T
}

impl<T> Deref for Scal<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.v
    }
}
/*
impl<T> Into<T> for Scal<T> {
    fn into(self) -> T {
        self.v
    }
}*/

impl<T> Mat<T> where T: Default + Clone{
    pub fn new(m: usize, n: usize) -> Self {
        Mat {
            m, n,
            vec: vec![T::default(); m * n]
        }
    }

    pub fn with_vec(v: Vec<Vec<T>>) -> Self {
        let mut m = 0;
        let mut n = 0;
        let mut vec = Vec::new();
        for (i, row) in v.into_iter().enumerate() {
            m += 1;
            //println!("m:{}", m);
            for col in row {
                if i == 0 {
                    n += 1;
                }
                //println!("n:{}", n);
                 vec.push(col);
            }
        }
        Mat {
            m, n, vec
        }
    }

    pub fn get(&self, m: usize, n:usize) -> Option<&T> {
        if m < self.m && n < self.n {
            Some(&self.vec[self.m * m + n])
        } else {
            None
        }
        
    }

    /*
    pub fn mAdd<A>(&self, right: &A)  -> Option<A> 
    where A : Mat<T> + Add<Output = T>
    {
        if (self.m, self.n) == (right.m, right.n) {
            let mut r = Mat {
                m: self.m, n: self.n, vec:Vec::new()
            };
            let v = &mut r.vec;
            self.vec.iter().zip(right.vec.iter()).for_each(|(a, b)| {
                v.push(*a + *b);
            });
            Some(r)

        } else {
            None
        }

    } 
    */
}

impl<T> std::ops::Index<usize> for Mat<T> 
where T:Default + Debug
{
    type Output = [T];
    fn index(&self, row: usize) -> &[T] {
        let start = row * self.n;
        &self.vec[start .. start + self.n]
    }
}

impl<T> std::ops::IndexMut<usize> for Mat<T> 
where T:Default + Debug
{
    fn index_mut(&mut self, row: usize) -> &mut [T] {
        let start = row * self.n;
        &mut self.vec[start .. start + self.n]
    }
}

impl<T> Display for Mat<T>  where T:Clone + Debug + Default
, Mat<T> : Index<usize, Output=[T]>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::from("");
        s = s + &format!("{}x{}\n", self.m, self.n);
        for i in 0..self.m {
           // println!("i:{}", i);
            s = s + &format!("{:?}\n", &self[i]);
        }
        write!(f, "{}", s)    

    }
}


impl<L, R, O> Add<Mat<R>> for Mat<L>
    where L: Add<R, Output=O> + Default+ Display, R: Default + Display, O: Default
{
    type Output = Mat<O>;
    fn add(self, rhs: Mat<R>) -> Self::Output {
        if (self.m, self.n) != (rhs.m, rhs.n) {
            panic!("add panic");
        }

        Mat{
            m: self.m, n: self.n,
            vec: self.vec.into_iter().zip(rhs.vec.into_iter()).map(|(a, b)| {
                a + b
            }).collect()
        }
    }
}

impl<R, L, O> Add<Scal<R>> for Mat<L>
    where L: Add<R, Output=O> + Default, O: Default, R: Default + Copy
{
    type Output = Mat<O>;
    fn add(self, rhs: Scal<R>) -> Self::Output {
        Mat {
            m: self.m, n: self.n,
            vec: self.vec.into_iter().map(|x| x + rhs.v).collect()
        }
    }
}

impl<R, L, O> Add<Mat<R>> for Scal<L>
    where L: Add<R, Output=O> + Default + Copy, R: Default, O: Default
{
    type Output = Mat<O>;
    fn add(self, rhs: Mat<R>) -> Self::Output {
        Mat {
            m: rhs.m, n:rhs.n,
            vec: rhs.vec.into_iter().map( |x| self.v + x ).collect()                                                                                                         
        }
    }
}

impl<L, R, O> Sub<Mat<R>> for Mat<L>
    where L: Sub<R, Output=O> + Default, R: Default, O:Default
{
    type Output = Mat<O>;
    fn sub(self, rhs: Mat<R>) -> Self::Output {
        if (self.m, self.n) != (rhs.m, rhs.n) {
            panic!("sub panic");
        }

        Mat {
            m: self.m, n:self.n,
            vec: self.vec.into_iter().zip(rhs.vec.into_iter()).map(|(a, b)| a - b).collect()
        }
    }
}

impl<L, R, O> Mul<Mat<R>> for Mat<L>
    where L: Mul<R, Output=O> + Default + Copy, R: Default + Copy, O:Default + Clone + Sum
    ,Self: Index<usize, Output=[L]> + IndexMut<usize, Output=[L]>
    ,Mat<R>: Index<usize, Output=[R]> + IndexMut<usize, Output=[R]>
    ,Mat<O>: Index<usize, Output=[O]> + IndexMut<usize, Output=[O]>
{
    type Output = Mat<O>;
    fn mul(self, rhs: Mat<R>) -> Self::Output {
        if self.n != rhs.m {
            panic!("mul panic");
        }

        let mut m : Self::Output = Mat::new(self.m, rhs.n);
        for i in 0..self.m {
            for j in 0..rhs.n {
                m[i][j] = self[i].iter().zip(
                             (0..self.n).map(|x| rhs[x][j])
                          ).map(|(a, b)| *a * b).sum();
            }
        }
        m
    }
}
