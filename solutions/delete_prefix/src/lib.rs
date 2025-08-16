 pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str>{
    s.strip_prefix(prefix)
}