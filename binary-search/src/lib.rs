pub fn find<T: PartialEq>(array: &[T], item: T) -> Option<usize> {
    find_by_ref(array, &item)
}

fn find_by_ref<T: PartialEq>(array: &[T], item: &T) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let index = array.len() / 2;

    if &array[index] == item {
        return Some(index);
    }

    if array.len() > 1 {
        return find_by_ref(&array[0..index], item).or_else(|| {
            find_by_ref(&array[(index + 1)..array.len()], item).map(|i| i + index + 1)
        });
    }

    None
}
