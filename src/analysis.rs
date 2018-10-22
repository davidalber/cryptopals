use std::collections::{HashMap, HashSet};

pub fn english_score(msg: &str) -> i32 {
    let mut score: i32 = 0;

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

    let mut vowels: HashSet<char> = HashSet::new();
    vowels.insert('a');
    vowels.insert('e');
    vowels.insert('i');
    vowels.insert('o');
    vowels.insert('u');

    // Get character counts.
    let mut char_counts: HashMap<char, u32> = HashMap::new();
    let mut vowel_count: usize = 0;
    let mut space_count: usize = 0;
    for c in msg.chars() {
        let c = c.to_lowercase().nth(0).unwrap();
        if c.is_alphabetic() {
            *char_counts.entry(c).or_insert(0) += 1
        }
        if vowels.contains(&c) {
            vowel_count += 1;
        }
        if c == ' ' {
            space_count += 1;
        }
    }

    let vowel_percent = vowel_count as f32 / (msg.len() - space_count) as f32;
    if vowel_percent < 0.15 {
        score -= 10;
    } else if vowel_percent > 0.3 {
        score += 50;
    } else if vowel_percent > 0.2 {
        score += 20;
    }

    if (msg.len() as f32 / space_count as f32) < 12. {
        score += 50;
    }

    for (c, count) in &char_counts {
        let char_percent = *count as f32 / (msg.len() - space_count) as f32;
        if let Some(avg_freq) = average_frequencies.get(&c) {
            let rel_diff = (avg_freq - char_percent).abs() / avg_freq;
            let delta = (1. / rel_diff).floor() as i32;
            if char_percent > *avg_freq - 0.01 {
                score += 10;
            }
            match rel_diff < 0.5 {
                true => score += delta,
                false => score -= delta,
            }
        } else if char_percent > 0.1 {
            score -= 5;
        }
    }

    score
}
