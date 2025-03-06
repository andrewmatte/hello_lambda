use std::arch::x86_64::*;

fn main() {
    let args: &Vec<String> = &(std::env::args().collect::<Vec<String>>());
    let filename: String = args[1].to_string();
    let vec_size: usize = args[2].parse::<usize>().unwrap();
    let vec: Vec<f32> = args[3..(vec_size + 3)]
        .into_iter()
        .map(|e| e.parse::<f32>().unwrap())
        .collect();
    let data: SearchData = load_bin_to_vec(&filename, vec_size);
    let results: Vec<Item> = search(data, vec);
    for result in results[0..1000].to_vec() {
        println!("{}", result.id);
    }
    std::process::exit(0);
}

#[target_feature(enable = "sse")]
pub unsafe fn manhattan(v1: &[f32], v2: &[f32]) -> f32 {
    let v1_reg = _mm_loadu_ps(v1.as_ptr());
    let v2_reg = _mm_loadu_ps(v2.as_ptr());

    // Perform the subtraction: xmm0 = xmm0 - xmm1
    let diff_reg = _mm_sub_ps(v1_reg, v2_reg); // xmm0 = v1 - v2

    // Calculate absolute values (clear the sign bit)
    let abs_reg = _mm_and_ps(diff_reg, _mm_set1_ps(0x7FFFFFFF as f32));

    // Horizontal add: sum x and y components (Manhattan distance)
    let sum_reg = _mm_add_ps(abs_reg, _mm_movehl_ps(abs_reg, abs_reg)); // Add y to x
    let final_sum_reg = _mm_add_ss(sum_reg, _mm_shuffle_ps(sum_reg, sum_reg, 1)); // Sum x + y

    // Extract the final Manhattan distance result
    _mm_cvtss_f32(final_sum_reg) // Convert the result to a scalar f32
}

fn load_bin_to_vec(vectors_filename: &String, vec_size: usize) -> SearchData {
    // This uses 2x the RAM of the filesize temporarily
    // to read in the data to RAM
    let bytes: Vec<u8> = std::fs::read(vectors_filename).unwrap();
    let file_length: usize = bytes.iter().count();
    let id_size: usize = 32;
    let row_size: usize = id_size + vec_size * 4 + 2 * 4;
    let nb_rows: usize = file_length / row_size;
    println!("{} entries", nb_rows);
    let mut id: &str;
    let mut vectors: Vec<f32>;
    let mut centroid: Vec<f32>;
    let mut geo: Vec<f32>;
    let mut row_start: usize;
    let float_size: usize = 4;
    let mut tmp_u8: [u8; 4];
    let mut vec_start: usize;
    let mut tmp_ids = Vec::<String>::new();
    let mut tmp_vectors = Vec::<Vec<f32>>::new();
    let mut tmp_geo = Vec::<Vec<f32>>::new();
    let mut tmp_centroid = Vec::<f32>::new();
    let mut data: SearchData = SearchData {
        vectors: tmp_vectors,
        ids: tmp_ids,
        geo: tmp_geo,
        centroid: tmp_centroid,
    };

    for row_index in 0..(nb_rows) {
        // Read ID
        row_start = row_index * row_size;
        id = std::str::from_utf8(&bytes[row_start..(row_start + id_size)]).unwrap();

        // Read vector
        vectors = Vec::with_capacity(vec_size);

        row_start += id_size;
        for vec_index in 0..vec_size {
            tmp_u8 = [0, 0, 0, 0];
            vec_start = vec_index * 4;
            for u_index in 0..4 {
                tmp_u8[u_index] = bytes[row_start + vec_start + u_index];
            }
            vectors.push(f32::from_ne_bytes(tmp_u8));
        }

        // Read coordinates
        row_start += vec_size * float_size;
        geo = Vec::with_capacity(2);
        for vec_index in 0..2 {
            tmp_u8 = [0; 4];
            vec_start = vec_index * 4;
            for u_index in 0..4 {
                tmp_u8[u_index] = bytes[row_start + vec_start + u_index];
            }
            geo.push(f32::from_ne_bytes(tmp_u8));
        }

        data.ids.push(id.to_owned());
        data.vectors.push(vectors);
        data.geo.push(geo);
    }

    return data;
}

fn search(app_state: SearchData, item: Vec<f32>) -> Vec<Item> {
    // Calculate distances
    let mut results: Vec<Item> = Vec::<Item>::new();
    let mut fl_dist: f32;
    let mut haver_dist: f32;

    for index in 0..app_state.vectors.len() {
        unsafe {
            fl_dist = manhattan(&item, &app_state.vectors[index]);
        }
        haver_dist = 0.0;
        // haver_dist = distances::haversine(&item.geoc, &app_state[&key.to_owned()].geo[index]);
        results.push(Item {
            id: app_state.ids[index].clone(),
            geo_dist: haver_dist,
            dist: fl_dist.sqrt(),
        });
    }

    // Sorting based on smallest distance
    glidesort::sort_by(&mut results, |a, b| a.dist.partial_cmp(&b.dist).unwrap());

    return results;
}

#[derive(Clone)]
struct Item {
    pub id: String,
    pub geo_dist: f32,
    pub dist: f32,
}

struct Items {
    pub items: Vec<Item>,
}

struct SearchData {
    pub vectors: Vec<Vec<f32>>,
    pub ids: Vec<String>,
    pub geo: Vec<Vec<f32>>,
    pub centroid: Vec<f32>,
}
