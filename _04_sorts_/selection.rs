pub fn sort(vec: &Vec<u32>) -> Vec<u32> {
    let mut v: Vec<u32> = vec.clone();
    let mut min_index: usize;
    let mut temp: u32;
    for i in 0..v.len() {
        min_index = i;
        for j in i..v.len() {
            if v[j] < v[min_index] {
                min_index = j;
            }
        }
        temp = v[min_index];
        v[min_index] = v[i];
        v[i] = temp;
    }

    v
}
