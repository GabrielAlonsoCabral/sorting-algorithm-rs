use std::time::Instant;

use lib::{
    bubble_sort, generate_random_array, insertion_sort, merge_sort, quick_sort, selection_sort,
};

fn main() {
    const ARRAY_SIZE: usize = 10000;
    const MIN_VALUE: i32 = 0;
    const MAX_VALUE: i32 = 10000;

    sort_with_execution_time(
        quick_sort,
        &mut generate_random_array(ARRAY_SIZE, MIN_VALUE, MAX_VALUE),
        "quick_sort",
    );

    sort_with_execution_time(
        selection_sort,
        &mut generate_random_array(ARRAY_SIZE, MIN_VALUE, MAX_VALUE),
        "selection_sort",
    );

    sort_with_execution_time(
        merge_sort,
        &mut generate_random_array(ARRAY_SIZE, MIN_VALUE, MAX_VALUE),
        "merge_sort",
    );

    sort_with_execution_time(
        bubble_sort,
        &mut generate_random_array(ARRAY_SIZE, MIN_VALUE, MAX_VALUE),
        "bubble_sort",
    );

    sort_with_execution_time(
        insertion_sort,
        &mut generate_random_array(ARRAY_SIZE, MIN_VALUE, MAX_VALUE),
        "insertion_sort",
    );
}

fn sort_with_execution_time<F>(sort_algorithm: F, arr: &mut Vec<i32>, algorithm_name: &str)
where
    F: Fn(&mut [i32]),
{
    let start: Instant = Instant::now();
    sort_algorithm(arr);

    println!(
        "{:?} algorithm takes {:?} ms to run",
        algorithm_name,
        start.elapsed().as_millis()
    );
}
