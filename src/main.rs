use codex::*;
use std::io::Write;
use std::fs::File;
use std::collections::HashMap;

static TABLE_URL: &'static str = "https://raw.githubusercontent.com/latex3/unicode-math/refs/heads/master/unicode-math-table.tex";

fn main() {
    let table = ureq::get(TABLE_URL)
        .call()
        .unwrap()
        .into_body()
        .read_to_string()
        .unwrap();
    let regex =
        regex::Regex::new("\\\\UnicodeMathSymbol\\{\"([A-F0-9]{5})\\}\\{\\\\([A-Za-z]+)").unwrap();

    let mut tex2uni = HashMap::new();
    for (_, [hex_point, name]) in regex.captures_iter(&table).map(|c| c.extract()) {
        let point = u32::from_str_radix(hex_point, 16).unwrap();
        let ch = char::from_u32(point).unwrap();
        tex2uni.insert(name, ch);
    }

    let mut uni2typ = HashMap::new();
    codex::SYM.iter().for_each(|(name, bind)| {
        if bind.deprecation.is_none() {
            match bind.def {
                Def::Symbol(Symbol::Single(symbol)) => {
                    uni2typ.insert(symbol, name.into());
                }
                Def::Symbol(Symbol::Multi(variants)) => {
                    uni2typ.insert(variants[0].1, name.into());
                    for (variant_name, variant_char) in variants {
                        let dot = if variant_name.len() > 0 { "." } else { "" };
                        uni2typ.insert(*variant_char, name.to_string() + dot + *variant_name);
                    }
                }
                Def::Module(_) => panic!("sub-modules not supported"),
            };
        }
    });

    let mut file = File::create("output.txt").expect(":(");
    for (tex, uni) in tex2uni.iter() {
        if let Some(typ) = uni2typ.get(uni) {
            writeln!(file, "{tex} = {typ}").expect(":(");
        }
    }
}
