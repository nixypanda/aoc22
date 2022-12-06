use std::collections::HashSet;
use std::hash::Hash;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn distinct_n_char_location(signal: &[char], n: usize) -> Option<usize> {
    for idx in 0..(signal.len() - n) {
        let slice = &signal[idx..(idx + n)];
        if has_unique_elements(slice) {
            return Some(idx + n);
        }
    }
    None
}

fn main() {
    let input = include_str!("../../data/day06.txt");
    let signal = input.chars().into_iter().collect::<Vec<char>>();

    let start_of_packet_marker = distinct_n_char_location(&signal, 4);
    println!("{:?}", start_of_packet_marker);
    let start_of_message_marker = distinct_n_char_location(&signal, 14);
    println!("{:?}", start_of_message_marker);
}
