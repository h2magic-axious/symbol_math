pub fn merge_vector<T>(v1: &Vec<T>, v2: &Vec<T>) -> Vec<T> {
    if v1.len() > v2.len() {
        return merge_vector(v2, v1);
    }

    let mut result = Vec::new();

    for i in 0..v1.len() {
        result.push(v1.get(i).unwrap() + v2.get(i).unwrap());
    }

    for i in v2.len()..v1.len() {
        result.push(v1.get(i).unwrap())
    }

    result
}