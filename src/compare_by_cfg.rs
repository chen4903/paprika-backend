use chrono::{DateTime, Utc};
use rand::{distr::Alphanumeric, Rng};
use serde::Serialize;
use std::{fs, path::Path, u32};
use tracing::debug;

use crate::{bytecode_to_image, cfg, constants::CLEAN_CFG_IMAGE_DATA, disassemble, image_analysis};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SimilarScore {
    pub score: f64,
    pub bytecode1: String,
    pub bytecode2: String,
    pub opcode1: String,
    pub opcode2: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct SerializableCfgResult {
    pub node_count: usize,
    pub edge_count: usize,
    pub nodes: Vec<String>,
    pub edges: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CompareCFGResult {
    pub cfg_result: Vec<SerializableCfgResult>,
    pub similar_score_result: Vec<SimilarScore>,
}

/// CFG-level similarity comparison. Process: Bytecode => CFG => Image => Loop comparison.
pub async fn compare_by_cfg(bytecode1: &str, bytecode2: &str) -> CompareCFGResult {
    debug!("Start to compare the image difference of cfg");

    let mut result: CompareCFGResult = CompareCFGResult {
        cfg_result: vec![],
        similar_score_result: vec![],
    };

    // bytecode1 needs to be longer to ensure the integrity of CFG comparison
    let (bytecode1, bytecode2) = if bytecode1.len() < bytecode2.len() {
        (bytecode1, bytecode2)
    } else {
        (bytecode2, bytecode1)
    };

    // 1. store the raw cfg result
    let bytecode1_cfg_result = cfg::build_cfg_from_bytecode(bytecode1).await.unwrap();
    let bytecode2_cfg_result = cfg::build_cfg_from_bytecode(bytecode2).await.unwrap();

    let nodes1 = cfg::get_nodes(bytecode1_cfg_result.clone());
    let edges1: Vec<(usize, usize)> = bytecode1_cfg_result
        .graph
        .edge_indices()
        .map(|e| {
            let (source, target) = bytecode1_cfg_result.graph.edge_endpoints(e).unwrap();
            (source.index(), target.index())
        })
        .collect();

    let nodes2 = cfg::get_nodes(bytecode2_cfg_result.clone());
    let edges2: Vec<(usize, usize)> = bytecode2_cfg_result
        .graph
        .edge_indices()
        .map(|e| {
            let (source, target) = bytecode2_cfg_result.graph.edge_endpoints(e).unwrap();
            (source.index(), target.index())
        })
        .collect();

    let cfg_result = vec![
        SerializableCfgResult {
            node_count: bytecode1_cfg_result.graph.node_count(),
            edge_count: bytecode1_cfg_result.graph.edge_count(),
            nodes: nodes1,
            edges: edges1,
        },
        SerializableCfgResult {
            node_count: bytecode2_cfg_result.graph.node_count(),
            edge_count: bytecode2_cfg_result.graph.edge_count(),
            nodes: nodes2,
            edges: edges2,
        },
    ];
    result.cfg_result = cfg_result;

    // 2. let's compare the difference of each cfg nodes
    let cfg1_nodes = cfg::get_nodes(bytecode1_cfg_result);
    let cfg2_nodes = cfg::get_nodes(bytecode2_cfg_result);

    // record the time as a component of the generated image's name.
    let now_utc: DateTime<Utc> = Utc::now();
    let formatted_utc = now_utc.format("%Y-%m-%d_%H-%M-%S").to_string();

    /*
        Strategy:
            Compare the images of each node from the two CFGs to find the highest similarity.
        Idea:
            Different nodes may generate images of varying sizes, then the program will panic with different size.
            If forced to standardize their sizes(With the longer size), significant differences in length could result
            in excessive blank space, artificially inflating the similarity score.

            To solve it, we only make a comparison when the absolute difference in the side lengths of the
            squares is either 0 or 1. In all other cases, the similarity is considered to be 0.0%.

            Generate images while calculating scores: Since we don't know the dimensions of the generated images,
            we first compare the lengths, and only if they meet the size requirements do we proceed to generate
            the images for comparison.
    */

    let mut score_result: Vec<SimilarScore> = vec![];
    let mut min_diff = u32::MAX;
    for node1_index in 0..cfg1_nodes.len() {
        let node1 = &cfg1_nodes[node1_index];
        // the two generated images need to be of the same size; otherwise, they cannot be compared.
        let size1 = bytecode_to_image::cal_appropriate_size((node1.len() / 2).try_into().unwrap());

        let random_string: String = generate_random_string();
        let node1_small_png_name =
            format!("output/{}/node1_small_{}.png", formatted_utc, random_string);
        let node1_large_png_name =
            format!("output/{}/node1_large_{}.png", formatted_utc, random_string);

        if let Some(parent) = Path::new(&node1_small_png_name).parent() {
            fs::create_dir_all(parent).unwrap();
        }

        // small
        match bytecode_to_image::transform_bytecode_to_image(&node1, size1) {
            Ok(img) => {
                img.save(node1_small_png_name.clone()).unwrap();
                debug!("`{}` is saved successfully", node1_small_png_name);
            }
            Err(e) => {
                debug!("Fail to save the `{}`: {}", node1_small_png_name, e);
            }
        }

        // large
        match bytecode_to_image::transform_bytecode_to_image(&node1, size1 + 1) {
            Ok(img) => {
                img.save(node1_large_png_name.clone()).unwrap();
                debug!("`{}` is saved successfully", node1_large_png_name);
            }
            Err(e) => {
                debug!("Fail to save the `{}`: {}", node1_large_png_name, e);
            }
        }

        let mut best_similar_score = 0.0;
        let mut best_similar_score_struct = SimilarScore {
            score: 0.0,
            bytecode1: "".to_string(),
            bytecode2: "".to_string(),
            opcode1: "".to_string(),
            opcode2: "".to_string(),
        };

        for node2_index in 0..cfg2_nodes.len() {
            let node2 = &cfg2_nodes[node2_index];
            let node2_png_name = format!("output/{}/node2_{}.png", formatted_utc, random_string);

            let mut size2 = bytecode_to_image::cal_appropriate_size(node2.len() / 2);
            let mut is_node1_small = true;

            // generate the image of node2 and compare with node1
            if (size1 as isize) - (size2 as isize) == -1 {
                // node1 large v.s. node2
                is_node1_small = false;
                match bytecode_to_image::transform_bytecode_to_image(&node2, size2) {
                    Ok(img) => {
                        img.save(node2_png_name.clone()).unwrap();
                        debug!("`{}` is saved successfully", node2_png_name);
                    }
                    Err(e) => {
                        debug!("Fail to save the `{}`: {}", node2_png_name, e);
                    }
                }
            } else if size1 == size2 {
                // node1 small v.s. node2
                match bytecode_to_image::transform_bytecode_to_image(&node2, size2) {
                    Ok(img) => {
                        img.save(node2_png_name.clone()).unwrap();
                        debug!("`{}` is saved successfully", node2_png_name);
                    }
                    Err(e) => {
                        debug!("Fail to save the `{}`: {}", node2_png_name, e);
                    }
                }
            } else if (size1 as isize) - (size2 as isize) == 1 {
                // node1 small v.s. node2's size+1
                size2 += 1;

                match bytecode_to_image::transform_bytecode_to_image(&node2, size2) {
                    Ok(img) => {
                        img.save(node2_png_name.clone()).unwrap();
                        debug!("`{}` is saved successfully", node2_png_name);
                    }
                    Err(e) => {
                        debug!("Fail to save the `{}`: {}", node2_png_name, e);
                    }
                }
            } else {
                // if the difference in the side lengths of the generated square images is greater than 1, then we will not compare them
                continue;
            }

            let node1_png_name = if is_node1_small {
                &node1_small_png_name
            } else {
                &node1_large_png_name
            };

            let similar_score =
                image_analysis::image_comparison_aggregator(node1_png_name, &node2_png_name);
            let diff = if node1_index > node2_index {
                (node1_index - node2_index) as u32
            } else {
                (node2_index - node1_index) as u32
            };

            if similar_score > best_similar_score {
                best_similar_score = similar_score;
                best_similar_score_struct = SimilarScore {
                    score: best_similar_score,
                    bytecode1: node1.clone(),
                    bytecode2: node2.clone(),
                    opcode1: disassemble::build_disassemble_from_bytecode(node1)
                        .await
                        .unwrap(),
                    opcode2: disassemble::build_disassemble_from_bytecode(node2)
                        .await
                        .unwrap(),
                };
            } else if similar_score == best_similar_score && diff < min_diff {
                min_diff = diff;
                best_similar_score_struct = SimilarScore {
                    score: best_similar_score,
                    bytecode1: node1.clone(),
                    bytecode2: node2.clone(),
                    opcode1: disassemble::build_disassemble_from_bytecode(node1)
                        .await
                        .unwrap(),
                    opcode2: disassemble::build_disassemble_from_bytecode(node2)
                        .await
                        .unwrap(),
                };
            }
        }

        // OK, we have find the best result in this round, we push it in the array
        score_result.push(best_similar_score_struct);
    }

    result.similar_score_result = score_result;
    debug!(
        "Succeed to compare the cfg! bytecode1 length: {}, bytecode2 length: {}",
        bytecode1.len(),
        bytecode2.len()
    );

    if CLEAN_CFG_IMAGE_DATA {
        let output_dir = "output";
        if let Err(e) = fs::remove_dir_all(output_dir) {
            eprintln!("Failed to delete directory {}: {}", output_dir, e);
        }
    }

    return result;
}

/// generate a random string
fn generate_random_string() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(12)
        .map(char::from)
        .collect()
}
