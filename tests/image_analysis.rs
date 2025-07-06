use paprika::image_analysis;

#[test]
fn test_image_comparison_aggregator() {
    let similar_score = image_analysis::image_comparison_aggregator("data/1.png", "data/2.png");
    println!("score: {}", similar_score);
}
