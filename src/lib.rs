use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Ошибга" => "Err",
        "ФсеОк" => "Ok",
        "Здрога" => "String",
        "ГежДаблица" => "HashMap",
        "Ошибка" => "Error",
        "Фозможно" => "Option",
        "Чдодо" => "Some",
        "Буздода" => "None",
        "Резульдад" => "Result",
        "Зебя" => "Self",
        "набизадь" => "println",
        "озданофидса" => "break",
        "азингронная" => "async",
        "бодождадь" => "await",
        "бедля" => "loop",
        "оддадь" => "move",
        "ящиг" => "crate",
        "недоздубный_год" => "unreachable_code",
        "гаг" => "as",
        "гонзданда" => "const",
        "гонфенция" => "trait",
        "обазно" => "unsafe",
        "ф" => "in",
        "из" => "from",
        "динамичезгий" => "dyn",
        "разбагофадь" => "unwrap",
        "гаг_ззылга" => "as_ref",
        "фнежний" => "extern",
        "небрафда" => "false",
        "фунгция" => "fn",
        "зупер" => "super",
        "фздафидь" => "insert",
        "получидь" => "get",
        "разрежидь" => "allow",
        "паниговадь" => "panic",
        "модуль" => "mod",
        "беременная" => "mut",
        "нофый" => "new",
        "где" => "where",
        "для" => "for",
        "болучидб_или_фздафидь_з" => "get_or_insert_with",
        "эл" => "main",
        "бубличная" => "pub",
        "фернуть" => "return",
        "реализофадь" => "impl",
        "ззылга" => "ref",
        "зрафнидь" => "match",
        "езли" => "if",
        "иначе" => "else",
        "зебя" => "self",
        "пуздь" => "let",
        "здадичезгая" => "static",
        "здругдура" => "struct",
        "бога" => "while",
        "избользовадб" => "use",
        "брефрадидь_ф" => "into",
        "бравда" => "true",
        "беречизление" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn buzdolang(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
