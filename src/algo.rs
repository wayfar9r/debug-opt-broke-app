use std::collections::HashSet;

/// Запоминаем пройденные элементы в сэте
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::with_capacity(values.len());
    let mut set = HashSet::new();
    for v in values {
        if !set.contains(v) {
            out.push(*v);
            set.insert(*v);
        }
    }
    out.sort();
    out
}

/// Вычисляем линейно
pub fn slow_fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        x => {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=x {
                let current = a + b;
                a = b;
                b = current;
            }
            b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::algo::slow_fib;

    #[test]
    fn test_slow_fib() {
        assert_eq!(slow_fib(0), 0);
        assert_eq!(slow_fib(1), 1);
        assert_eq!(slow_fib(2), 1);
        assert_eq!(slow_fib(3), 2);
        assert_eq!(slow_fib(10), 55);
    }
}
