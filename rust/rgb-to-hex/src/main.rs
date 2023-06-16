/// Returns a hex string from RGB values.
/// 
/// # Arguments
/// 
/// * `r` - The red value of the color.
/// * `g` - The green value of the color.
/// * `b` - The blue value of the color.
fn rgb(r: i32, g: i32, b: i32) -> String {
    // Create a new empty string to hold the hex value.
    let mut hex = String::new();
    // Create a vector of the RGB values.
    let mut rgb = vec![r, g, b];
    // Iterate over the RGB values.
    for i in 0..3 {
        // Check for out of bounds values.
        if rgb[i] < 0 {
            rgb[i] = 0;
        } else if rgb[i] > 255 {
            rgb[i] = 255;
        }
        // Push the hex value of the RGB value to the hex string.
        // Read about formatting strings here:
        // https://doc.rust-lang.org/std/fmt/
        hex.push_str(&format!("{:02X}", rgb[i]));
    }
    hex
}

macro_rules! compare {
    ( $got : expr, $expected : expr ) => {
        if $got != $expected { panic!("Got: {}\nExpected: {}\n", $got, $expected); }
    };
}

fn main() {
    compare!(rgb(0, 0, 0), "000000");
    compare!(rgb(1, 2, 3), "010203");
    compare!(rgb(255, 255, 255), "FFFFFF");
    compare!(rgb(254, 253, 252), "FEFDFC");
    compare!(rgb(-20, 275, 125), "00FF7D");
}
