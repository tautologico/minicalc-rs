pub mod lexer;
pub mod parser;

use lexer::Buffer;
use lexer::TipoToken;

fn main() {
    let teste1 = "777 (4 + 3)";

    let mut buffer1 = Buffer::cria_com_string(teste1);

    println!("Interpretador Minicalc");
    println!("Teste do analisador sintatico");

    let arvore = parser::analise_sintatica(&mut buffer1);

    println!("Arvore sintatica: {:?}", arvore);

    println!("Analise lexica finalizada");
}
