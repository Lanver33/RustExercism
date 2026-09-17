pub fn annotate(garden: &[&str]) -> Vec<String> {
    let result: Vec<Vec<bool>> = garden
        .iter()
        .map(|row| row.chars().map(|character| character == '*').collect())
        .collect();

    Vec::new()
}
