/// Takes a pixel value from the radar picture and returns a value between 0.0 and 1.0,
/// indicating the precipitation intensity
pub fn px_to_precipitation(px: image::Rgba<u8>) -> f32 {
    static POSSIBLE_COLORS_AND_PRECIPITATION_VALUES: [((i32, i32, i32), f32); 15] = [
        ((8, 70, 254), 15.0),
        ((0, 120, 254), 18.0),
        ((0, 174, 253), 21.0),
        ((0, 220, 254), 24.0),
        ((4, 216, 131), 27.0),
        ((66, 235, 66), 30.0),
        ((108, 249, 0), 33.0),
        ((184, 250, 0), 36.0),
        ((249, 250, 1), 39.0),
        ((254, 198, 0), 42.0),
        ((254, 132, 0), 45.0),
        ((255, 62, 1), 48.0),
        ((211, 0, 0), 51.0),
        ((181, 3, 3), 54.0),
        ((203, 0, 204), 57.0),
    ];

    if px[3] < 128 {
        0.0
    } else {
        // Find closest color by Euclidean distance
        let mut best_precipitation_value = 0.0;
        let mut best_distance_squared = std::i32::MAX;
        for x in POSSIBLE_COLORS_AND_PRECIPITATION_VALUES.iter() {
            let (color, precipitation_value) = x;
            let distance_squared = (color.0 - px[0] as i32).pow(2)
                + (color.1 - px[1] as i32).pow(2)
                + (color.2 - px[2] as i32).pow(2);
            if distance_squared < best_distance_squared {
                best_distance_squared = distance_squared;
                best_precipitation_value = *precipitation_value;
            }
        }
        best_precipitation_value / 57.0
    }
}

#[cfg(test)]
mod tests {
    use super::px_to_precipitation;

    #[test]
    fn test_px_to_precipitation() {
        assert_eq!(px_to_precipitation(image::Rgba::<u8>([0, 0, 0, 0])), 0.0);
        assert_eq!(
            px_to_precipitation(image::Rgba::<u8>([203, 0, 204, 255])),
            1.0
        );
        assert_eq!(
            px_to_precipitation(image::Rgba::<u8>([66, 235, 66, 255])),
            0.5263158
        );
    }
}
