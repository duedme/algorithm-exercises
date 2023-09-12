fn divisibleSumPairs(n: i32, k: i32, s: &[i32]) -> i32 {
    let mut result = 0;

    for index in 0..s.len() {
        for jindex in index + 1..s.len() {
            if (s[index] + s[jindex]) % k == 0 {
                result = result + 1
            }
        }
    }

    result
}

fn main() {
    let n=6;
    let k = 3;
    let ar = &[1, 3, 2, 6, 1, 2];
    assert_eq!(divisibleSumPairs(n, k, ar), 5);
}
