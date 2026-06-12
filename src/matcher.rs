use crate::db::Entry;

fn is_subsequence(needle: &str, haystack: &str) -> bool {
    let mut chars = haystack.chars();
    needle.chars().all(|n| chars.any(|h| h == n))
}

pub fn matches(entry: &Entry, keywords: &[String]) -> bool {
    let command_lower = entry.command.to_lowercase();
    keywords.iter().all(|kw| {
        let kw_lower = kw.to_lowercase();
        is_subsequence(&kw_lower, &command_lower)
    })
}

pub fn filter<'a>(entries: Vec<&'a Entry>, keywords: &[String]) -> Vec<&'a Entry> {
    if keywords.is_empty() {
        return entries;
    }
    entries.into_iter().filter(|e| matches(e, keywords)).collect()
}
