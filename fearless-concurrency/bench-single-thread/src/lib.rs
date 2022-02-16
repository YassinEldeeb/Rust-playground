pub fn parse_md(content: String) -> Vec<String> {
    let lines = content.lines();

    let mut results = vec![];

    for i in lines {
        let v = identify_h(i);

        if let Some(val) = v {
            results.push(val);
        }
    }

    results
}

fn identify_h(line: &str) -> Option<String> {
    if line.starts_with("#") {
        let mut size = 0;
        for i in line.chars() {
            if i.to_string() == "#" {
                size += 1;
            } else {
                break;
            }
        }

        if size <= 6 {
            return Some(format!("<h{}>{}</h{}>", size, &line[size + 1..], size));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use comrak::{markdown_to_html, ComrakOptions};
    use serde::{Deserialize, Serialize};
    use std::{
        collections::HashMap,
        fmt::Write,
        fs,
        ops::Add,
        time::{Duration, Instant},
    };

    #[derive(Serialize, Deserialize, Debug)]
    struct Run {
        index: i32,
        duration: Duration,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Bench {
        avg: String,
        sum: Duration,
        num_of_iter: usize,
        iterations: Vec<Run>,
    }

    #[test]
    fn bench() {
        let mut results: Vec<Run> = vec![];
        for i in 1..=100 {
            let now = Instant::now();
            markdown_to_html(
                "# Hello

## ## Tricky

## World

### How is it going

####### Hello
",
                &ComrakOptions::default(),
            );
            let elapsed = now.elapsed();

            results.push(Run {
                index: i,
                duration: elapsed,
            })
        }

        let sum: Duration = results.iter().map(|e| e.duration).sum();
        let avg = sum.as_nanos() / (results.len() as u128);

        let bench = Bench {
            avg: format!("{} nano/iter", avg),
            num_of_iter: results.len(),
            iterations: results,
            sum,
        };

        let serialized = serde_json::to_string(&bench).unwrap();

        fs::write("bench2.json", &serialized).unwrap();
    }
}
