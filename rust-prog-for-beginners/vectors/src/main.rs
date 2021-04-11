struct TestScore {
    score: i32,
}

fn main() {
    let test_scores = vec![
        TestScore { score: 99 },
        TestScore { score: 52 },
        TestScore { score: 23 },
        TestScore { score: 97 },
    ];

    for test in test_scores {
        println!("Test score is {}", test.score);
    }
}
