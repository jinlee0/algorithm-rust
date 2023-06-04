use std::io;
use std::fmt::Write;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace().flat_map(str::parse::<usize>);
    let mut output = String::new();
    let n = input.next().unwrap();
    let mut dp = vec![vec![usize::MAX; 501];501];
    let mut arr = vec![0; 501];
    let mut sum = vec![0; 501];
        for _ in 0..n {
        let n = input.next().unwrap();
        for i in 1..=n as usize {
            arr[i] = input.next().unwrap();
            sum[i] = sum[i-1] + arr[i];
        }
        writeln!(output, "{}", merge(&arr, &mut dp, &sum, 1, n)).unwrap();
        reset(&mut dp, n);
    }
    println!("{}", output);
}

fn reset(dp: &mut Vec<Vec<usize>>, n: usize) {
    for i in 1..=n {
        for j in 1..=n {
            dp[i][j] = usize::MAX;
        }
    }
}

fn merge(arr: &Vec<usize>, dp: &mut Vec<Vec<usize>>, sum: &Vec<usize>, s: usize, e: usize) -> usize {
    if dp[s][e] != usize::MAX {
        return dp[s][e];
    } else if s == e {
        return 0;
    } else if s + 1 == e {
        return arr[s] + arr[e];
    }
    
    for i in s..e {
        let left = merge(arr, dp, sum, s, i);
        let right = merge(arr, dp, sum, i+1, e);
        dp[s][e] = dp[s][e].min(left + right);
    }
    dp[s][e] += sum[e] - sum[s-1];
    dp[s][e]
}

#[test]
fn test() {
    let input = vec![1, 21, 3, 4, 5, 35, 5, 4, 3, 5, 98, 21, 14, 17, 32];
    let n = 15;
    let mut dp = vec![vec![usize::MAX;501];501];
    let mut arr = vec![0; 501];
    let mut sum = vec![0; 501];
    for i in 1..=n as usize {
        arr[i] = input[i-1];
        sum[i] = sum[i-1] + arr[i];
    }
    merge(&arr, &mut dp, &sum, 1, n);
}