fn test_reaction(a: u8, b: u8) -> bool {
    (if b > a { b - a } else { a - b }) == (b'a' - b'A')
}

fn get_reactions(polymer: &[u8], reactions: &mut Vec<usize>) -> bool {
    reactions.clear();

    let mut i = 0;
    while i + 1 < polymer.len() {
        if test_reaction(polymer[i], polymer[i + 1]) {
            reactions.push(i);
            i += 1;
        }
        i += 1;
    }

    !reactions.is_empty()
}

fn react_once(output: &mut [u8], removals: &mut Vec<usize>) -> Option<usize> {
    if !get_reactions(&output[..], removals) {
        return None;
    }

    let num_removed = removals.len() * 2;

    removals.push(output.len());

    let mut swap_to = removals[0];

    // shift everything down between each set of removals
    for removal_pair in removals.windows(2) {
        for x in removal_pair[0] + 2..removal_pair[1] {
            output.swap(x, swap_to);
            swap_to += 1;
        }
    }

    Some(output.len() - num_removed)
}

pub fn react(polymer: &mut [u8]) -> usize {
    let mut removals = vec![];
    let mut output_length = polymer.len();
    while let Some(new_length) = react_once(&mut polymer[..output_length], &mut removals) {
        output_length = new_length;
    }
    output_length
}

fn react_splits(left: &[u8], right: &[u8]) -> usize {
    let total_len = std::cmp::min(left.len(), right.len());

    for x in 0..total_len {
        if !test_reaction(left[left.len() - x - 1], right[x]) {
            return x;
        }
    }

    total_len
}

pub fn react_par(polymer: &mut [u8]) -> usize {
    if polymer.len() <= 1024 {
        return react(polymer);
    }

    let split_point = polymer.len() / 2;

    let (left, right) = polymer.split_at_mut(split_point);

    let mut left_range = 0..left.len();
    let mut right_range = 0..right.len();

    let split_reactions = react_splits(&left[left_range.clone()], &right[right_range.clone()]);
    left_range.end -= split_reactions;
    right_range.start += split_reactions;

    let (left_len, right_len) = rayon::join(
        || react_par(&mut left[left_range.clone()]),
        || react_par(&mut right[right_range.clone()]),
    );

    left_range.end = left_range.start + left_len;
    right_range.end = right_range.start + right_len;

    let split_reactions = react_splits(&left[left_range.clone()], &right[right_range.clone()]);
    left_range.end -= split_reactions;
    right_range.start += split_reactions;

    let past_left = &mut polymer[left_range.end..];
    let gap_space = right_range.start + split_point - left_range.end;

    past_left.rotate_left(gap_space);

    left_range.count() + right_range.count()
}
