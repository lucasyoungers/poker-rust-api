use rand::Rng;
use itertools::Itertools;

// Shuffles a vector in-place.
#[allow(dead_code)]
pub fn shuffle_mut<T>(vec: &mut Vec<T>) {
    let mut rng = rand::thread_rng();
    for i in 0..vec.len() {
        let j = rng.gen_range(0..=i);
        vec.swap(i, j);
    }
}

// Returns a shuffled copy of a vector.
#[allow(dead_code)]
pub fn shuffle<T>(vec: &Vec<T>) -> Vec<T> where T: Clone {
    let mut vec_copy = vec.clone();
    shuffle_mut(&mut vec_copy);
    vec_copy
}

// Samples n elements from a vector without replacement.
#[allow(dead_code)]
pub fn sample<T>(vec: &Vec<T>, n: usize) -> Vec<T> where T: Clone {
    if n > vec.len() {
        panic!("Cannot sample more elements than vector contains.");
    }
    let vec_shuffled = shuffle(vec);
    let len = vec_shuffled.len();
    vec_shuffled[len-n..len].to_vec()
}

// Returns a random element from a vector.
#[allow(dead_code)]
pub fn choose<T>(vec: &Vec<T>) -> T where T: Copy {
    vec[rand::thread_rng().gen_range(0..vec.len())]
}

// Samples n elements from a vector with replacement.
#[allow(dead_code)]
pub fn choices<T>(vec: &Vec<T>, n: usize) -> Vec<T> where T: Copy {
    (0..n).map(|_| choose(vec)).collect()
}

// Returns all k-combinations of a given vector.
#[allow(dead_code)]
pub fn combinations<T>(vec: &Vec<T>, k: usize) -> Vec<Vec<T>> where T: Clone {
    if k > vec.len() {
        panic!("Cannot generate k-combinations for k greater than vector length.");
    }
    vec.iter().combinations(k).map(|c| c.into_iter().cloned().collect()).collect()
}
