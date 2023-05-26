/// Returns the ordering of the ants leaving the bridge.
///
/// # Arguments
///
/// * `ants` - A string of characters representing the ants on the bridge.
/// * `terrain` - A string of characters representing the terrain of the
///              bridge. The '.' character represents a gap in the bridge. The
///              '-' character represents a solid part of the bridge.
///              The length of the string is the length of the bridge.
fn ant_bridge(ants: &str, terrain: &str) -> String {
    let gaps = gaps(terrain);
    // If there are less ants than gaps, ants will be unable to cross at all.
    if ants.len() < gaps {
        return String::from("");
    }
    // If there are more ants than gaps, ants will be able to cross, with the
    // first ants filling the gaps and the remaining ants crossing first before
    // being followed by the ants that filled the gaps.
    return format!("{}{}", &ants[ants.len()-gaps..], &ants[..ants.len()-gaps]);
}

/// Returns the number of ants that can fill all gaps in the terrain.
///
/// # Arguments
///
/// * `terrain` - A string of characters representing the terrain of the
///             bridge. The '.' character represents a gap in the bridge. The
///             '-' character represents a solid part of the bridge.
///             The length of the string is the length of the bridge.
fn gaps(terrain: &str) -> usize {
    // Splits the terrain string on the '-' character, then iterates over the 
    // resulting vector of strings.
    // Return the sum of the lengths of all strings with non-zero length, plus 
    // 2 times the number of non-zero length strings.
    return terrain.split('-')
        .filter(|s| s != &"")
        .map(|s| s.len() + 2).sum();
}

fn main() {
    let test_ants = "GFEDCBA";
    let test_terrain = "-----------...-----------";
    println!("{}", ant_bridge(test_ants, test_terrain));
}
