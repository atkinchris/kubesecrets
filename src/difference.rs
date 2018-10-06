use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;

pub fn difference<'a, T: Eq + Hash>(
  before: &'a Vec<T>,
  after: &'a Vec<T>,
) -> (Vec<&'a T>, Vec<&'a T>, Vec<&'a T>) {
  let before_set: HashSet<&T> = HashSet::from_iter(before);
  let after_set: HashSet<&T> = HashSet::from_iter(after);

  let unchanged = before_set.intersection(&after_set).cloned().collect();
  let added = after_set.difference(&before_set).cloned().collect();
  let deleted = before_set.difference(&after_set).cloned().collect();

  (unchanged, added, deleted)
}

#[cfg(test)]
mod tests {
  use super::difference;

  #[test]
  fn test_unchanged() {
    let before = vec!["a", "b", "c"];
    let after = vec!["a", "b", "c"];
    let mut result = difference(&before, &after);

    // Sorting as internal hashset order is not guaranteed
    assert_eq!(result.0.sort(), vec![&"a", &"c", &"b"].sort());
  }

  #[test]
  fn test_added() {
    let before = vec!["a", "b", "c"];
    let after = vec!["a", "b", "c", "d"];
    let result = difference(&before, &after);

    assert_eq!(result.1, vec![&"d"]);
  }

  #[test]
  fn test_deleted() {
    let before = vec!["a", "b", "c", "d"];
    let after = vec!["a", "b", "c"];
    let result = difference(&before, &after);

    assert_eq!(result.2, vec![&"d"]);
  }
}
