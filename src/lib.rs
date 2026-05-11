pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter(|&&x| x % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов. Буфер намеренно не освобождается,
/// что приведёт к утечке памяти (Valgrind это покажет).
/// Мы могли бы использователь std::alloc::dealloc(raw, Layout::array::<u8>(len)
/// для освобождения памяти и оставив функцию небезопасной, но в этом
/// нет необходимости
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|&&x| x != 0).count()
}

/// Небрежная нормализация строки: удаляем пробелы и приводим к нижнему регистру,
/// но игнорируем повторяющиеся пробелы/табуляции внутри текста.
pub fn normalize(input: &str) -> String {
    input.replace(' ', "").to_lowercase()
}

/// Логическая ошибка: усредняет по всем элементам, хотя требуется учитывать
/// только положительные. Деление на длину среза даёт неверный результат.
pub fn average_positive(values: &[i64]) -> f64 {
    let (sum, count) = values
        .iter()
        .filter(|&&x| x > 0)
        .fold((0f64, 0u32), |(sum, count), &x| (sum + x as f64, count + 1));
    if count == 0 { 0.0 } else { sum / count as f64 }
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    *b + *b
}

#[cfg(test)]
mod tests {
    use crate::{average_positive, leak_buffer, sum_even, use_after_free};

    #[test]
    fn test_sum_even() {
        assert_eq!(sum_even(&[-1, 0, 2, 5]), 2);
        assert_eq!(sum_even(&[]), 0);
        assert_eq!(sum_even(&[-1]), 0);
        assert_eq!(sum_even(&[100]), 100);
    }

    #[test]
    fn test_average_positive() {
        assert_eq!(average_positive(&[]), 0.0);
        assert_eq!(average_positive(&[-1000, -2, 0, 2, 1]), 1.5);
        assert_eq!(average_positive(&[1]), 1.0);
        assert_eq!(average_positive(&[-5]), 0.0);
    }

    #[test]
    fn test_leak_buffer() {
        assert_eq!(leak_buffer(&[0]), 0);
        assert_eq!(leak_buffer(&[1, 255]), 2);
        assert_eq!(leak_buffer(&[0, 0]), 0);
        assert_eq!(leak_buffer(&[]), 0);
    }

    #[test]
    fn test_use_after_free() {
        use_after_free();
    }
}
