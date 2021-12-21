pub fn find_low_points(height_map: &[&[u8]]) -> Vec<u8> {
    for j in 0..height_map.len() {
        for k in 0..height_map[j].len() {
            let c = height_map[j][k];
        }
    }

    vec![]
}

#[cfg(test)]
mod answers {
    use super::*;

    static SAMPLE: &[&[u8]] = &[
        "2199943210".as_bytes(),
        "3987894921".as_bytes(),
        "9856789892".as_bytes(),
        "8767896789".as_bytes(),
        "9899965678".as_bytes(),
    ];

    #[test]
    fn example1() {
        find_low_points(SAMPLE);
        assert!(false);
    }
}
