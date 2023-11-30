use std::{cmp::Ordering, env::args};
use std::io::{stdout, Write, BufWriter};

use rand::{thread_rng, Rng};

fn random_sort(mut v: Vec<char>, rng: &mut impl Rng) -> Vec<char> {
    v.sort_by(|_, _| {
        if rng.gen::<bool>() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    v
}

fn main() {
    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    let content = args()
        .nth(1)
        .unwrap_or("大変申し訳ありません。厳しく".to_string());
    let content_vec: Vec<_> = content.chars().collect();
    let mut rng = thread_rng();
    let mut current_vec = random_sort(content_vec.clone(), &mut rng);
    let mut times = 0;
    let mut res = String::with_capacity(2048);
    while content_vec != current_vec {
        current_vec = random_sort(current_vec, &mut rng);
        if res.len() > 1024 {
            out.write_all(res.as_bytes()).unwrap();
            res.clear();
        } else {
            for c in current_vec.iter() {
                res.push(*c);
            }
            res.push('\n')
        }
        times += 1;
    }
    out.write_all(res.as_bytes()).unwrap();
    out.write_all(format!("{}回かかりました。\n", times).as_bytes()).unwrap();
}
