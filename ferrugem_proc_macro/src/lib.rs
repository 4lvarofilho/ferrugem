use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn substituir_identificador(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let nova_str = match ident_str.as_str() {
        "Erro" => "Err",
        "Ok" => "Ok",
        "Texto" | "Cadeia" => "String",
        "MapaHash" => "HashMap",
        "Padrao" => "Default",
        "ErroTipo" => "Error",
        "Opcao" => "Option",
        "Algum" => "Some",
        "Nenhum" => "None",
        "Resultado" => "Result",
        "Auto" => "Self",
        "escreva" | "imprima" | "amostre" => "println",
        "pare" => "break",
        "assincrono" => "async",
        "aguarde" | "pera" => "await",
        "laco" => "loop",
        "mova" => "move",
        "caixa" => "crate",
        "codigo_inacessivel" => "unreachable_code",
        "como" => "as",
        "constante" => "const",
        "interface" => "trait",
        "inseguro" => "unsafe",
        "em" => "in",
        "de" => "from",
        "dinamico" => "dyn",
        "desembrulhar" => "unwrap",
        "padrao" => "default",
        "como_ref" => "as_ref",
        "es" => "io",
        "externo" => "extern",
        "falso" => "false",
        "funcao" | "fn" => "fn",
        "super" => "super",
        "inserir" | "meta"=> "insert",
        "obter" | "pegue" => "get",
        "permitir" | "deixe" => "allow",
        "merda" | "caralho" | "pane" | "mizéra" | "desgraça" => "panic",
        "modulo" => "mod",
        "mut" | "mutavel" => "mut",
        "novo" => "new",
        "onde" => "where",
        "para" => "for",
        "obter_ou_inserir_com" | "eu_boto_ou_num_boto" => "get_or_insert_with",
        "principal" => "main",
        "publico" | "pub" => "pub",
        "que" | "an" | "eoq" => None?,
        "retorne" => "return",
        "implementacao" | "impl" => "impl",
        "ref" => "ref",
        "combine" => "match",
        "se" => "if",
        "senao" => "else",
        "este" => "self",
        "seja" | "var" => "let",
        "estatico" => "static",
        "estrutura" => "struct",
        "espera" => "expect",
        "enquanto" => "while",
        "use" | "usando" => "use",
        "para_dentro" | "ai_dento" => "into",
        "verdadeiro" => "true",
        "enumeracao" => "enum",
        "Grupo" => "Group",
        "Identificador" => "Ident",
        "FluxoDeTokens" => "TokenStream",
        "ArvoreDeTokens" => "TokenTree",
        "para_texto" => "to_string",
        "como_texto" => "as_str",
        "escopo" => "span",
        "Vetor" => "Vec",
        "fluxo" | "no_flow_por_onde_a_gente_passa_é_show" => "stream",
        "adicionar" => "push",
        "estender" => "extend",
        "delimitador" => "delimiter",
        "Pontuacao" => "Punct",
        "Literal" => "Literal",
        "macro_procedural" => "proc_macro",
        
        _ => &ident_str,
    };

    let novo_ident = Ident::new(nova_str, ident.span());
    Some(TokenTree::Ident(novo_ident))
}

fn substituir_arvore(tok: TokenTree, saida: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            substituir_fluxo(group.stream(), &mut group_elem);
            let mut novo_fluxo = TokenStream::new();
            novo_fluxo.extend(group_elem);
            saida.push(TokenTree::Group(Group::new(group.delimiter(), novo_fluxo)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = substituir_identificador(ident) {
                saida.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            saida.push(tok);
        }
    }
}

fn substituir_fluxo(ts: TokenStream, saida: &mut Vec<TokenTree>) {
    for tok in ts {
        substituir_arvore(tok, saida)
    }
}

#[proc_macro]
pub fn ferrugem(item: TokenStream) -> TokenStream {
    let mut resultado = Vec::new();
    substituir_fluxo(item, &mut resultado);
    let mut saida = TokenStream::new();
    saida.extend(resultado);
    saida
}