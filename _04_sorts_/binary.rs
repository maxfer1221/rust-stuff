pub fn search(vec: &Vec<u32>, target: u32, l: usize, r: usize) -> isize {
    let i = (l+r)/2;
    let c = vec[i];
    if target == c {
        return i as isize;
    } else if r-l == 0{
        return -1;
    } else if target > c {
        return search(vec, target, i+1, r);
    } else {
        return search(vec, target, l, i-1);
    }
}
