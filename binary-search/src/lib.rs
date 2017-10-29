pub fn find<T: PartialOrd>(array: &[T], item: T) -> Option<usize> {
    find_by_ref(array, &item)
}

fn find_by_ref<T: PartialOrd>(array: &[T], item: &T) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let index = array.len() / 2;

    if *item == array[index] {
        Some(index)
    } else if *item < array[index] {
        find_by_ref(&array[0..index], item)
    } else {
        find_by_ref(&array[(index + 1)..array.len()], item).map(|i| i + index + 1)
    }
}
