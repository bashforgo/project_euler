use std::f32;

use project_euler::integer;

type Size = u128;

pub fn solve() -> String {
    let mut top = vec![];
    let mut bot = vec![];

    for a in 1..=9 {
        for b in 0..=9 {
            let ab = 10 * a + b;
            for x in a..=9 {
                for y in 0..=9 {
                    let xy = 10 * x + y;
                    if ab >= xy {
                        continue;
                    }
                    let abxy = ab as f32 / xy as f32;

                    if b == x {
                        let ay = a as f32 / y as f32;
                        if (abxy - ay).abs() < f32::EPSILON {
                            top.push(ab);
                            bot.push(xy);
                        }
                    }
                }
            }
        }
    }

    let t = top.into_iter().product::<Size>();
    let b = bot.into_iter().product::<Size>();
    let d = integer::gcd(t, b);

    (b / d).to_string()
}
