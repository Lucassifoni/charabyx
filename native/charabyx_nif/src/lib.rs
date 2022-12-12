use charabia::{Language, Script, SeparatorKind, TokenKind, Tokenize};
use rustler::{Atom, Env, NifResult, NifStruct};

#[derive(NifStruct)]
#[module = "Charabyx.NifToken"]
struct NifToken {
    kind: rustler::Atom,
    lemma: String,
    script: rustler::Atom,
    char_start: usize,
    char_end: usize,
    language: Option<rustler::Atom>,
}

fn to_token_kind(env: &Env, x: TokenKind) -> NifResult<rustler::Atom> {
    let k = match x {
        TokenKind::Word => "word",
        TokenKind::StopWord => "stopword",
        TokenKind::Unknown => "unknown",
        TokenKind::Separator(a) => match a {
            SeparatorKind::Hard => "separator_hard",
            SeparatorKind::Soft => "separator_soft",
        },
    };
    Atom::from_str(*env, k)
}

fn to_language(env: &Env, language: Option<Language>) -> Option<rustler::Atom> {
    match language {
        None => None,
        Some(l) => Some(to_default_atom(
            env,
            Atom::from_str(*env, format!("{:?}", l).as_str()),
            "default_language",
        )),
    }
}
fn to_script(env: &Env, x: Script) -> NifResult<rustler::Atom> {
    Atom::from_str(*env, format!("{:?}", x).as_str())
}

fn to_default_atom(env: &Env, a: NifResult<rustler::Atom>, string: &str) -> rustler::Atom {
    let default_atom = Atom::from_str(*env, string).unwrap();
    match a {
        Err(_) => default_atom,
        Ok(b) => b,
    }
}

#[rustler::nif]
fn tokenize<'a>(env: Env<'a>, input: String) -> Vec<NifToken> {
    return input
        .as_str()
        .tokenize()
        .into_iter()
        .map(|x| NifToken {
            kind: to_default_atom(&env, to_token_kind(&env, x.kind), "default_kind"),
            lemma: x.lemma().to_string(),
            script: to_default_atom(&env, to_script(&env, x.script), "default_script"),
            char_end: x.char_end,
            char_start: x.char_start,
            language: to_language(&env, x.language),
        })
        .collect();
}

rustler::init!("Elixir.CharabyxNif", [tokenize]);
