#[derive(Debug)]
pub enum LambdaTerm {
    Variable(String),
    Abstraction(String, Box<LambdaTerm>),
    Appliction(Box<LambdaTerm>, Box<LambdaTerm>),
}

pub fn parse(src: &str) -> Option<(LambdaTerm, &str)> {
    cap(parse_variable, src)
        .or_else(|| cap(parse_abstraction, src))
        .or_else(|| cap(parse_application, src))
}

fn cap<'a, 'b, T>(
    prs: fn(&'a str) -> Option<(T, &'b str)>,
    src: &'a str,
) -> Option<(T, &'b str)> {
    prs(src.trim_start())
}

fn parse_variable_name(src: &str) -> Option<(&str, &str)> {
    let t = src.trim_start_matches(|c: char| (c == '_' || c.is_alphabetic()));
    let x = &src[..src.len() - t.len()];
    if x.is_empty() {
        None
    } else {
        Some((x, t))
    }
}

fn parse_variable(src: &str) -> Option<(LambdaTerm, &str)> {
    parse_variable_name(src)
        .and_then(|(x, t)| Some((LambdaTerm::Variable(x.to_string()), t)))
}

fn parse_abstraction(src: &str) -> Option<(LambdaTerm, &str)> {
    if src.starts_with('\\') {
        let src = &src[1..];
        if let Some((x, t)) = cap(parse_variable_name, src) {
            let t = t.trim_start();
            if t.starts_with('.') {
                parse(&t[1..]).and_then(|(m, t)| {
                    Some((
                        LambdaTerm::Abstraction(x.to_string(), Box::new(m)),
                        t,
                    ))
                })
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_application(src: &str) -> Option<(LambdaTerm, &str)> {
    parse(src).and_then(|(m, t)| {
        cap(parse, t).and_then(|(n, t)| {
            Some((LambdaTerm::Appliction(Box::new(m), Box::new(n)), src))
        })
    })
}
