pub fn sort(vec: &Vec<u32>) -> Vec<u32> {
    let mut v: Vec<u32> = vec.clone();
    for i in 0..v.len()-1 {
        for j in 0..v.len()-i-1 {
            if v[j] > v[j+1] {
                let t = v[j];
                v[j] = v[j+1];
                v[j+1] = t;
            }
        }
    }
    v
}
