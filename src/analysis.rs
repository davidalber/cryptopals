use std::collections::HashMap;

pub fn english_score(msg: &str) -> u32 {
    let mut score: u32 = 0;

    let mut average_frequencies: HashMap<char, f32> = HashMap::new();
    average_frequencies.insert('e', 0.1202);
    average_frequencies.insert('t', 0.091);
    average_frequencies.insert('a', 0.0812);
    average_frequencies.insert('o', 0.0768);
    average_frequencies.insert('i', 0.0731);
    average_frequencies.insert('n', 0.0695);
    average_frequencies.insert('s', 0.0628);
    average_frequencies.insert('r', 0.0602);
    average_frequencies.insert('h', 0.0592);

    // Get character counts.
    let mut char_counts: HashMap<char, f32> = HashMap::new();
    for c in msg.chars() {
        let c = c.to_lowercase().nth(0).unwrap();
        if c.is_alphabetic() {
            *char_counts.entry(c).or_insert(0.) += 1.
        }
    }

    for (c, count) in &char_counts {
        let char_percent = count / msg.len() as f32;
        if let Some(avg_freq) = char_counts.get(&c) {
            let rel_diff = (avg_freq - char_percent).abs() / avg_freq;
            let delta = (1. / (1. - rel_diff)).floor() as u32;
            match (avg_freq - char_percent).abs() / avg_freq > 0.5 {
                true => score += delta,
                false => score -= delta,
            }
        } else {
            if char_percent > 0.05 {
                score -= 4;
            }
        }
    }

    score
}
