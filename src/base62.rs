const BASE: u64 = 62;
const ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

async fn encode(mut num: u64) -> String {
    if num == 0 {
        return "0".into();
    }
    let mut result_str = String::new();
    while num > 0 {
        let remainder = (num % BASE) as usize;
        num /= BASE;
        result_str.push(ALPHABET[remainder] as char);
    }
    return result_str.chars().rev().collect();
}

pub async fn prefix_encode(num: u64) -> String {
    let mut result = encode(num).await;
    result = format!("i{}", result);
    return result;
}