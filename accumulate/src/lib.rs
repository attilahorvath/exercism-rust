pub fn map_function<T>(array: Vec<T>, f: &impl Fn(T) -> T) -> Vec<T> {
    let mut result = Vec::with_capacity(array.len());

    for i in array {
        result.push(f(i));
    }

    result
}

pub fn map_closure<T>(array: Vec<T>, f: impl Fn(T) -> T) -> Vec<T> {
    map_function(array, &f)
}
