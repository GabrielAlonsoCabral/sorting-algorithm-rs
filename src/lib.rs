use rand::{thread_rng, Rng};

pub fn generate_random_array(n: usize, min: i32, max: i32) -> Vec<i32> {
    let mut rng: rand::rngs::ThreadRng = thread_rng();
    (0..n).map(|_| rng.gen_range(min..=max)).collect()
}

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index: usize = partition(arr);

    let (left, right) = arr.split_at_mut(pivot_index);
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let len: usize = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    let len: usize = arr.len();
    for i in 1..len {
        let mut j: usize = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

pub fn merge_sort<T: Ord + Clone + std::marker::Copy>(arr: &mut [T]) {
    let len: usize = arr.len();
    if len <= 1 {
        return;
    }

    let mid: usize = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merged: Vec<T> = Vec::with_capacity(len);
    let (mut i, mut j) = (0, mid);

    while i < mid && j < len {
        if arr[i] <= arr[j] {
            merged.push(arr[i].clone());
            i += 1;
        } else {
            merged.push(arr[j].clone());
            j += 1;
        }
    }

    if i < mid {
        merged.extend_from_slice(&arr[i..mid]);
    }
    if j < len {
        merged.extend_from_slice(&arr[j..]);
    }

    arr.copy_from_slice(&merged);
}

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    let len: usize = arr.len();
    for i in 0..len {
        let mut min_index: usize = i;
        for j in i + 1..len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
    }
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len: usize = arr.len();
    let pivot_index: usize = len / 2;
    arr.swap(pivot_index, len - 1);

    let mut i = 0;
    for j in 0..(len - 1) {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}
