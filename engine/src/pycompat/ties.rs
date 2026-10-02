//! Python's `max` and `min`, which keep the first of equal elements.
//!
//! `Iterator::max_by_key` keeps the last, so the engine's "newest record by
//! `(updated_at, id)`" or "latest instant" would pick a different element
//! wherever two compare equal, and the element kept matters: a timestamp
//! written `...Z` and one written `...+00:00` are equal instants but not
//! the same string. The engine's `clippy.toml` bans the `Iterator::max` and
//! `Iterator::min` families so that these are used instead.

use std::cmp::Ordering;

/// `max(items, key=key)`: the first element whose key is largest. Each key
/// is computed once, in order. `None` where Python raises on an empty
/// sequence.
pub fn py_max_by_key<I, K, F>(items: I, key: F) -> Option<I::Item>
where
    I: IntoIterator,
    K: Ord,
    F: FnMut(&I::Item) -> K,
{
    first_where(items, key, Ordering::Greater)
}

/// `min(items, key=key)`: the first element whose key is smallest.
pub fn py_min_by_key<I, K, F>(items: I, key: F) -> Option<I::Item>
where
    I: IntoIterator,
    K: Ord,
    F: FnMut(&I::Item) -> K,
{
    first_where(items, key, Ordering::Less)
}

/// `max(items)` under a comparison: an element replaces the one kept only
/// if it compares greater.
pub fn py_max_by<I, F>(items: I, mut compare: F) -> Option<I::Item>
where
    I: IntoIterator,
    F: FnMut(&I::Item, &I::Item) -> Ordering,
{
    keep_first(items, |item, kept| compare(item, kept) == Ordering::Greater)
}

/// `min(items)` under a comparison: an element replaces the one kept only
/// if it compares less.
pub fn py_min_by<I, F>(items: I, mut compare: F) -> Option<I::Item>
where
    I: IntoIterator,
    F: FnMut(&I::Item, &I::Item) -> Ordering,
{
    keep_first(items, |item, kept| compare(item, kept) == Ordering::Less)
}

fn first_where<I, K, F>(items: I, mut key: F, wins: Ordering) -> Option<I::Item>
where
    I: IntoIterator,
    K: Ord,
    F: FnMut(&I::Item) -> K,
{
    let mut kept: Option<(K, I::Item)> = None;
    for item in items {
        let k = key(&item);
        let replaces = match &kept {
            None => true,
            Some((best, _)) => k.cmp(best) == wins,
        };
        if replaces {
            kept = Some((k, item));
        }
    }
    kept.map(|(_, item)| item)
}

fn keep_first<I, F>(items: I, mut replaces: F) -> Option<I::Item>
where
    I: IntoIterator,
    F: FnMut(&I::Item, &I::Item) -> bool,
{
    let mut kept: Option<I::Item> = None;
    for item in items {
        let better = match &kept {
            None => true,
            Some(current) => replaces(&item, current),
        };
        if better {
            kept = Some(item);
        }
    }
    kept
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_instants_keep_the_first_spelling() {
        // Two spellings of one instant: Python's max keeps the one met
        // first, and the engine then writes that string back out.
        let stamps = ["2026-01-01T00:00:00Z", "2026-01-01T00:00:00+00:00"];
        let instant = |s: &&str| {
            s.trim_end_matches("Z")
                .trim_end_matches("+00:00")
                .to_string()
        };
        assert_eq!(py_max_by_key(stamps, instant), Some(stamps[0]));
        assert_eq!(py_min_by_key(stamps, instant), Some(stamps[0]));
        #[allow(clippy::disallowed_methods)]
        let rust = stamps.into_iter().max_by_key(instant);
        assert_eq!(rust, Some(stamps[1]), "Iterator::max_by_key keeps the last");
    }

    #[test]
    fn comparison_forms_keep_the_first_of_equals() {
        let items = [(1, 'a'), (2, 'b'), (2, 'c'), (0, 'd'), (0, 'e')];
        assert_eq!(py_max_by(items, |a, b| a.0.cmp(&b.0)), Some((2, 'b')));
        assert_eq!(py_min_by(items, |a, b| a.0.cmp(&b.0)), Some((0, 'd')));
        assert_eq!(py_max_by(Vec::<i32>::new(), |a, b| a.cmp(b)), None);
    }
}
