use std::collections::HashMap;
use std::hash::Hash;

pub fn find_duplicates<'a, T>(iter: T) -> Option<Vec<T::Item>>
where
  T: IntoIterator,
  T::Item: Eq + Hash + Clone,
{
  let groups = iter.into_iter().fold(HashMap::new(), |mut m, c| {
    *m.entry(c).or_insert(0) += 1;
    m
  });

  let duplicates: Vec<T::Item> = groups
    .iter()
    .filter_map(|(item, &count)| if count > 1 { Some(item.clone()) } else { None })
    .collect();

  if duplicates.is_empty() {
    None
  } else {
    Some(duplicates)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_duplicates() {
    let with_duplicates = vec!["a", "b", "c", "d", "a"];
    let without_duplicates = vec!["a", "b", "c", "d", "e"];

    assert_eq!(find_duplicates(with_duplicates), Some(vec!["a"]));
    assert_eq!(find_duplicates(without_duplicates), None);
  }
}
