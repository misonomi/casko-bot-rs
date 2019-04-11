use lazy_static::lazy_static;
use regex::Regex;

pub fn has_prefix(mes: &str) -> Option<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\s*\^\^~\s*(.*)").expect("wrong prefix!!!!!!!!!");
    }
    if let Some(text) = RE.captures(mes) {
        return text.get(1).map(|m| m.as_str());
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_has_prefix() {
        thp_core("not accepted", None);
        thp_core("", None);
        thp_core("^^~", Some(""));
        thp_core("^^~ok", Some("ok"));
        thp_core("^^~  ok", Some("ok"));
        thp_core("  ^^~  ok", Some("ok"));
        thp_core("^^~^^~", Some("^^~"));
    }

    fn thp_core(str_in: &str, str_out: Option<&str>) {
        assert_eq!(has_prefix(str_in), str_out);
    }
}