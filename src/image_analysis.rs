use image_compare::{gray_similarity_structure, Algorithm};
use tracing::debug;

/// Aggregate the result of the two image comparison tools
pub fn image_comparison_aggregator(image_path_1: &str, image_path_2: &str) -> f64 {
    let result1 = tool_image_compare(image_path_1, image_path_2);
    let result2 = 1.0 - (tool_dssim(image_path_1, image_path_2));

    let average = (result1 + result2) / 2.0;
    debug!(
        "image comparison aggregator, the average similar score is: {}",
        average
    );

    return average;
}

/// https://github.com/ChrisRega/image-compare/tree/main
fn tool_image_compare(image_path_1: &str, image_path_2: &str) -> f64 {
    // Load two images
    let image_one = image::open(image_path_1).expect("Could not find image1.png");
    let image_two = image::open(image_path_2).expect("Could not find image2.png");

    // Convert the image to grayscale format
    let gray_image_one = image_one.to_luma8();
    let gray_image_two = image_two.to_luma8();

    // Comparing images using MSSIM algorithm
    let result =
        gray_similarity_structure(&Algorithm::MSSIMSimple, &gray_image_one, &gray_image_two)
            .expect("Images had different dimensions");

    debug!(
        "image comparison tool: ChrisRega/image-compare. Score: {}",
        result.score
    );

    return result.score;
}

/// https://github.com/kornelski/dssim/tree/main
fn tool_dssim(image_path_1: &str, image_path_2: &str) -> f64 {
    let attr = dssim::Dssim::new();

    let prof_jpg = dssim::load_image(&attr, image_path_1).unwrap();
    let prof_png = dssim::load_image(&attr, image_path_2).unwrap();

    let (diff, _) = attr.compare(&prof_jpg, prof_png);
    debug!(
        "image comparison tool: kornelski/dssim. Score: {}",
        1.0 - diff
    );

    return diff.into();
}
