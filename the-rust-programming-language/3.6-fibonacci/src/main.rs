use std::collections::HashMap;

fn main() {
    println!("result {}", fibonacci(70));
}

// fn fibonacci(n: u32) -> u32 {
//     if n == 0 {
//         return 0;
//     }

//     if n == 1 {
//         return 1;
//     }

//     return fibonacci(n - 1) + fibonacci(n - 2);
// }

fn fibonacci(n: u64) -> u64 {
    return cached_fibonacci(n, &mut HashMap::new());
}

fn cached_fibonacci(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if let Some(&result) = cache.get(&n) {
        return result;
    } else {
        let result = match n {
            0 => 0,
            1 => 1,
            _ => cached_fibonacci(n - 1, cache) + cached_fibonacci(n - 2, cache),
        };
        cache.insert(n, result);
        return result;
    }
}
