use std::collections::HashMap;
use std::fmt::Display;
use std::fmt;

pub struct Zilla<'a, T: Formatzilla>(pub &'a T);

impl<'a, T: Formatzilla> Display for Zilla<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt_zilla(f)
    }
}

pub trait Formatzilla {
    fn fmt_zilla(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn zilla(&self) where Self: Sized {
        println!("{}", Zilla(self));
    }
}

impl<K, V> Formatzilla for HashMap<K, V>
where
    K: Display + Ord,
    V: Display,
{
    fn fmt_zilla(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut pairs: Vec<_> = self.iter().collect();
        pairs.sort_by_key(|(k, _)| k.to_string());

        let max_key_len = pairs.iter()
            .map(|(k, _)| k.to_string().len())
            .max()
            .unwrap_or(0);

        writeln!(f, "HashMap:")?;
        for (k, v) in pairs {
            let k_str = k.to_string();
            let padding = max_key_len - k_str.len();
            writeln!(f, "    {k_str}:{} {v},", " ".repeat(padding))?;
        }

        Ok(())
    }
}

impl<T> Formatzilla for [T]
where
    T: Display,
{
    fn fmt_zilla(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[T]:")?;
        for (i, v) in self.iter().enumerate() {
            writeln!(f, "    {}. {v},", i + 1)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_map_is_valid() {
        let mut hashmap = HashMap::new();
        hashmap.insert("key", "value");
        hashmap.insert("test", "true");

        assert_eq!(format!("{}", Zilla(&hashmap)), "HashMap:\n    key:  value,\n    test: true,\n");
    }
}