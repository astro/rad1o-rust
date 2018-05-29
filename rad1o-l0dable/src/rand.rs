use table::table;

/// Get a random number
pub fn get_random() -> u32 {
    (table().getRandom)()
}
