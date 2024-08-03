pub fn is_match(s: String, p: String) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let p = p.chars().collect::<Vec<_>>();

    let p = parse(&p);
    is_match_inner(&s, &p)
}

struct MatchUnit {
    v: char,
    is_multi: bool,
}

impl MatchUnit {
    fn is_match(&self, c: char) -> bool {
        self.v == c || self.v == '.'
    }
}

fn parse(p: &[char]) -> Vec<MatchUnit> {
    let mut res: Vec<MatchUnit> = vec![];
    for &c in p {
        if c == '*' {
            res.last_mut().unwrap().is_multi = true;
        } else {
            res.push(MatchUnit {
                v: c,
                is_multi: false,
            });
        }
    }
    res
}

fn is_match_inner(s: &[char], p: &[MatchUnit]) -> bool {
    if s.is_empty() && p.is_empty() {
        return true;
    }

    if p.is_empty() && !s.is_empty() {
        return false;
    }

    let m = &p[0];
    if !m.is_multi {
        if !s.is_empty() && m.is_match(s[0]) {
            return is_match_inner(&s[1..], &p[1..]);
        } else {
            return false;
        }
    }

    for i in 0..=s.len() {
        if i >0 && !m.is_match(s[i - 1]) {
            return false;
        }

        if is_match_inner(&s[i..], &p[1..]) {
            return true;
        }
    }

    false
}
