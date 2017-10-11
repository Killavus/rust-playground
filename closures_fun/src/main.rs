use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

struct MemoizedFn<F, P, R>
    where F: Fn(P) -> R,
          P: Copy + Hash + Eq,
          R: Copy
{
    calculation: F,
    values: HashMap<P, R>,
}

impl<F, P, R> MemoizedFn<F, P, R>
    where F: Fn(P) -> R,
          P: Copy + Hash + Eq,
          R: Copy
{
    fn new(calculation: F) -> MemoizedFn<F, P, R> {
        MemoizedFn {
            calculation: calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: P) -> R {
        {
            let possible_value = self.values.get(&arg);

            if let Some(&value) = possible_value {
                return value;
            }
        }

        let result = (self.calculation)(arg);
        self.values.insert(arg, result);
        result
    }
}

fn generate_workout(intensity: i32, random_number: i32) {
    let mut expensive_closure = MemoizedFn::new(|num| {
                                                    println!("calculating slowly...");
                                                    thread::sleep(Duration::from_secs(2));
                                                    num
                                                });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!",
                     expensive_closure.value(intensity))
        }
    }
}


fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    //simulated_expensive_calculation(12);
}
