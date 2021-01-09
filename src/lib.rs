use proc_macro::*;


#[proc_macro]
pub fn atom(input: TokenStream) -> TokenStream {
    let mut iter = IntoIterator::into_iter(input);
    let input_token = iter.next().expect("atom identifier");

    if let Some(unexpected) = iter.next() {
        panic!(format!("unexpected tokens: {}", unexpected));
    }

    let name = if let TokenTree::Ident(ident) = input_token {
        ident.to_string()
    } else {
        panic!("expected identifier");
    };

    let atom_id = atom_id(&name);

    atom_tokens(atom_id)
}

const SYMBOL_COUNT: usize = 27;

fn atom_char_idx(ch: char) -> usize {
    match ch {
        'a' => { 0 },
        'b' => { 1 },
        'c' => { 2 },
        'd' => { 3 },
        'e' => { 4 },
        'f' => { 5 },
        'g' => { 6 },
        'h' => { 7 },
        'i' => { 8 },
        'j' => { 9 },
        'k' => { 10 },
        'l' => { 11 },
        'm' => { 12 },
        'n' => { 13 },
        'o' => { 14 },
        'p' => { 15 },
        'q' => { 16 },
        'r' => { 17 },
        's' => { 18 },
        't' => { 19 },
        'u' => { 20 },
        'v' => { 21 },
        'w' => { 22 },
        'x' => { 23 },
        'y' => { 24 },
        'z' => { 25 },
        '_' => { 26 },
        c => { panic!(format!("unexpected character: {}", c)) }
    }
}

fn atom_id(name: &str) -> usize {
    name.chars()
        .fold(0, |acc, item| SYMBOL_COUNT * acc + atom_char_idx(item))
}

fn atom_tokens(id: usize) -> TokenStream {
    let atom_id: Vec<TokenTree> = vec!(
        Ident::new("id", Span::call_site()).into(),
        Punct::new(':', Spacing::Alone).into(),
        Literal::usize_unsuffixed(id).into(),
    );

    let mut inner_tokens = TokenStream::new();
    inner_tokens.extend(atom_id);

    let atom: Vec<TokenTree> = vec!(
        Ident::new("thespian", Span::call_site()).into(),
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new("erl", Span::call_site()).into(),
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new("atom", Span::call_site()).into(),
        Punct::new(':', Spacing::Joint).into(),
        Punct::new(':', Spacing::Alone).into(),
        Ident::new("Atom", Span::call_site()).into(),
        Group::new(Delimiter::Brace, inner_tokens).into(),
    );

    let mut tokens = TokenStream::new();
    tokens.extend(atom);

    tokens
}