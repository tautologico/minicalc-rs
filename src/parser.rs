//mod lexer;

// expressao eh um dos seguintes casos:
// - constante inteira (valor)
// - soma (op1 e op2)
// - multiplicacao (op1 e op2)

use crate::lexer::Buffer;
use crate::lexer::proximo_token;
use crate::lexer::TipoToken;

#[derive(Debug, PartialEq, Eq)]
pub enum Expressao {
    Constante(i64),
    Soma(Box<Expressao>, Box<Expressao>),
    Mult(Box<Expressao>, Box<Expressao>)
}

pub fn analise_sintatica(buffer: &mut Buffer) -> Expressao {
    let tok = proximo_token(buffer);

    if tok.tipo != TipoToken::Print {
        panic!("Token nao esperado: {:?} na linha {}", tok, buffer.linha)
    }

    Expressao::Constante(42)
}
