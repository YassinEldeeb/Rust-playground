use std::{collections::HashMap, hash::Hash, thread, time::Duration};

struct Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Hash + Eq + Clone,
    V: Clone,
{
    expensive_closure: T,
    cache: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where
    T: Fn(U) -> V,
    U: Hash + Eq + Clone,
    V: Clone,
{
    fn new(closure: T) -> Self {
        Cacher {
            expensive_closure: closure,
            cache: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        let value = self.cache.get(&arg);

        match value {
            Some(v) => (*v).clone(),
            None => {
                let v = (self.expensive_closure)(arg.clone());
                self.cache.insert(arg.clone(), v.clone());

                (self.cache.get(&arg).unwrap()).clone()
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_colsure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_colsure.value(intensity.clone())
        );
        println!("Next, do {} situps!", expensive_colsure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_colsure.value(intensity)
            );
        }
    }
}
