// ignore this part
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Write;
// ok now read

/// Actually good printing for maps and vectors.
/// Ditch {:?}. {:#?} too.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use agp::ActualGoodPrinting;
///
/// let mut map = HashMap::new();
/// map.insert("status", "ok");
/// map.insert("code", "200");
///
/// map.agp();
/// ```
pub trait ActualGoodPrinting {
    /// Prints the contents to standard output.
    /// - For HashMaps: prints in the format `K: V`, line by line.
    /// - For Vecs/arrays: prints in the format `N. item`, line by line (1-indexed).
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use agp::ActualGoodPrinting;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("status", "ok");
    /// map.insert("code", "200");
    ///
    /// map.agp();
    /// ```

    // todone AKA it's done: maybe add separate comments for vecs and standard arrays
    fn agp(&self);
    /// Returns a String containing the result.
    /// - For HashMaps: returns in the format `K: V`, line by line.
    /// - For Vecs/arrays: returns in the format `N. item`, line by line (1-indexed).
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use agp::ActualGoodPrinting;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("status", "ok");
    /// map.insert("code", "200");
    ///
    /// let result = map.return_agp();
    ///
    /// assert!(result.contains("status: ok"));
    /// assert!(result.contains("code: 200"));
    /// ```
    fn return_agp(&self) -> String;
}

impl<K, V> ActualGoodPrinting for HashMap<K, V>
where
    K: Display,
    V: Display,
{
    fn agp(&self) {
        for (k, v) in self {
            println!("{k}: {v}");
        }
    }

    fn return_agp(&self) -> String {
        let mut result = String::with_capacity(self.len() * 32);
        for (k, v) in self {
            writeln!(result, "{k}: {v}").unwrap();
        }
        result
    }
}

impl<T> ActualGoodPrinting for [T] // catches Vec and standard arrays
where
    T: Display,
{
    fn agp(&self) {
        for (index, item) in self.iter().enumerate() {
            println!("{}. {item}", index + 1);
        }
    }

    fn return_agp(&self) -> String {
        // preallocate so that it doesn't have to allocate and allocate
        let mut result = String::with_capacity(self.len() * 32);
        for (index, item) in self.iter().enumerate() {
            writeln!(result, "{}. {item}", index + 1).unwrap(); // unwrap() on a String will never fail
        }
        result
    }
}

// TODO: add BTreeMap impl (same as HashMap, just swap the type + add doc comments)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agp_compiles_and_prints() {
        let mut map = HashMap::new();
        map.insert("status", "ok");

        map.agp();
    }

    #[test]
    fn test_empty_map() {
        let map: HashMap<String, String> = HashMap::new();

        assert_eq!(map.return_agp(), "");
    }

    #[test]
    fn test_return_agp_content() {
        let mut map = HashMap::new();
        map.insert("status", "ok");
        map.insert("code", "200");

        let result = map.return_agp();

        assert!(result.contains("status: ok\n"));
        assert!(result.contains("code: 200\n"));

        let line_count = result.lines().count();
        assert_eq!(line_count, 2);
    }

    #[test]
    fn test_numeric_types() {
        let mut map = HashMap::new();
        map.insert(404, 12.5);
        map.insert(500, 99.9);

        let result = map.return_agp();

        assert!(result.contains("404: 12.5\n"));
        assert!(result.contains("500: 99.9\n"));
    }

    #[test]
    fn test_multiline_values() {
        let mut map = HashMap::new();
        map.insert("error", "Line 1\nLine 2");

        let result = map.return_agp();

        assert!(result.contains("error: Line 1\nLine 2\n"));
    }

    #[test]
    fn test_large_map_capacity() {
        let mut map = HashMap::new();
        for i in 0..1000 {
            map.insert(i, i * 2);
        }

        let result = map.return_agp();

        assert_eq!(result.lines().count(), 1000);
        assert!(result.contains("500: 1000\n"));
    }

    #[test]
    fn test_fixed_size_array() {
        let arr = [10, 20, 30]; // Traditional stack array, not a Vec!
        let result = arr.return_agp();

        assert_eq!(result, "1. 10\n2. 20\n3. 30\n");
    }
}

