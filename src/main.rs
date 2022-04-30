pub mod lexer;

use lexer::Buffer;
use lexer::TipoToken;

fn main() {
    let teste1 = "(+*43257)";

    let mut buffer1 = Buffer::cria_com_string(teste1);

    println!("Interpretador Minicalc");
    println!("Testando analisador lexico");

    let mut tok = lexer::proximo_token(&mut buffer1);
    while tok.tipo != TipoToken::Eof {
        println!("token: {:?}", tok);
        tok = lexer::proximo_token(&mut buffer1);
    }

    println!("Analise lexica finalizada");
}
