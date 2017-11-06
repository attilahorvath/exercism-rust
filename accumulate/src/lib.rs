pub fn map_function<T>(array: Vec<T>, f: &Fn(T) -> T) -> Vec<T> {
    let mut result = Vec::with_capacity(array.len());

    for i in array {
        result.push(f(i));
    }

    result
}

pub fn map_closure<T, F: Fn(T) -> T>(array: Vec<T>, f: F) -> Vec<T> {
    map_function(array, &f)
}
