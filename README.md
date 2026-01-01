# CodeWars-Human-Readable-Time-5-kyu---Passed-
Write a function, which takes a non-negative integer (seconds) as input and returns the time in a human-readable format (HH:MM:SS)

HH = hours, padded to 2 digits, range: 00 - 99
MM = minutes, padded to 2 digits, range: 00 - 59
SS = seconds, padded to 2 digits, range: 00 - 59
The maximum time never exceeds 359999 (99:59:59)

You can find some examples in the test fixtures.


TEST CASES
#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};
    use super::make_readable;
    
    fn reference_solution(se: u32) -> String {
        let h = se / 3600;
        let m = (se%3600) / 60;
        let s = se - 3600 * h - 60 * m;
        format!("{h:0>2}:{m:0>2}:{s:0>2}")
    }
    
    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
    
    fn dotest(s: u32, expected: &str) {
        assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
    }

    #[test]
    fn fixed_tests() {
        dotest(0, "00:00:00");
        dotest(59, "00:00:59");
        dotest(60, "00:01:00");
        dotest(3599, "00:59:59");
        dotest(3600, "01:00:00");
        dotest(86399, "23:59:59");
        dotest(86400, "24:00:00");
        dotest(359999, "99:59:59");
    }
    
    #[test]
    fn random_tests() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let s = rng.gen_range(0..360_000);
            dotest(s, &reference_solution(s));
        }
    }
}
