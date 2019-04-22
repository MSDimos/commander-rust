#[doc(hidden)]
#[derive(Debug, Eq, PartialEq)]
pub enum PatternType {
    Word,
    Short, // such as -abc
    Long, // such as --recursive
    Stmt, //  such as --output="/home/dist",
    Others, // something others
    Init, // initial value
}


#[doc(hidden)]
#[derive(Debug, Eq, PartialEq)]
pub struct Pattern<'a> {
    full: &'a str,
    pub groups: Vec<&'a str>,
    pub ty: PatternType,
}

impl<'a> Pattern<'a> {
    pub fn match_str(full: &str) -> Pattern {
        let mut reg = Pattern::new(full);

        if Pattern::is_word(full) {
            reg.ty = PatternType::Word;
            reg.groups = vec![full];
        } else if Pattern::is_short(full) {
            reg.ty = PatternType::Short;
            reg.groups = vec![&full[1..]]
        } else if Pattern::is_long(full) {
            reg.ty = PatternType::Long;
            reg.groups = vec![&full[2..]]
        } else if Pattern::is_stmt(full) {
            let idx = full.find("=").unwrap_or(2);
            let key = &full[2..idx];
            let value = &full[(idx + 1)..];

            reg.ty = PatternType::Stmt;
            reg.groups = vec![key, value];
        } else {
            reg.ty = PatternType::Others;
        }

        reg
    }

    // same as /\w+/
    pub fn is_word(s: &str) -> bool {
        s.chars().all(|c| char::is_ascii_alphabetic(&c))
    }

    pub fn is_short(s: &str) -> bool {
        s.len() >= 2 && s.starts_with("-") && Pattern::is_word(&s[1..])
    }

    pub fn is_long(s: &str) -> bool {
        s.len() >= 3 && s.starts_with("--") && Pattern::is_word(&s[2..])
    }

    pub fn is_stmt(s: &str) -> bool {
        s.len() > 4 &&
            s.starts_with("--") &&
            s.contains("=") &&
            !s.ends_with("=")
    }

    pub fn new(full: &'a str) -> Pattern<'a> {
        Pattern {
            full,
            groups: vec![],
            ty: PatternType::Init,
        }
    }
}

#[cfg(test)]
mod test {
    

    #[test]
    fn can_match() {
        use super::Pattern;
        assert!(Pattern::is_word("yes"));
        assert!(Pattern::is_short("-f"));
        assert!(Pattern::is_long("--recursive"));
        assert!(Pattern::is_stmt("--output=yes"));

        assert!(!Pattern::is_word("/path"));
        assert!(!Pattern::is_short("--/path"));
        // don't support like this
        assert!(!Pattern::is_long("--long-item"));
        assert!(!Pattern::is_stmt("--outputis="));
    }
    
    #[test]
    fn groups_check() {
        use super::{ Pattern, PatternType };
        assert_eq!(Pattern {
            full: "name",
            ty: PatternType::Word,
            groups: vec!["name"]
        }, Pattern::match_str("name"));

        assert_eq!(Pattern {
            full: "-abc",
            ty: PatternType::Short,
            groups: vec!["abc"],
        }, Pattern::match_str("-abc"));

        assert_eq!(Pattern {
            full: "--long",
            ty: PatternType::Long,
            groups: vec!["long"],
        }, Pattern::match_str("--long"));

        assert_eq!(Pattern {
            full: "--output=./path/",
            ty: PatternType::Stmt,
            groups: vec!["output", "./path/"],
        }, Pattern::match_str("--output=./path/"));

        assert_eq!(Pattern {
            full: "/path/",
            ty: PatternType::Others,
            groups: vec![],
        }, Pattern::match_str("/path/"));
    }
}