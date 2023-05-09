pub fn calculate_correlation_coefficient(data: Vec<(u64, u64)>) -> f64 {
    // https://en.wikipedia.org/wiki/Pearson_correlation_coefficient
    // Calculate the mean of x and y
    let (mean_x, mean_y): (f64, f64) = data.iter().fold((0.0, 0.0), |(sum_x, sum_y), &(x, y)| {
        (sum_x + x as f64, sum_y + y as f64)
    });
    let n = data.len() as f64;
    let mean_x = mean_x / n;
    let mean_y = mean_y / n;

    // Calculate the covariance and variances
    let covariance: f64 = data
        .iter()
        .map(|&(x, y)| ((x as f64 - mean_x) * (y as f64 - mean_y)))
        .sum();
    let variance_x: f64 = data.iter().map(|&(x, _)| (x as f64 - mean_x).powi(2)).sum();
    let variance_y: f64 = data.iter().map(|&(_, y)| (y as f64 - mean_y).powi(2)).sum();

    // Calculate the correlation coefficient
    let correlation: f64 = covariance / (variance_x.sqrt() * variance_y.sqrt());

    correlation
}
