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

#[derive(Debug, PartialEq, Eq)]
enum Operador {
    Soma,
    Mult
}

pub fn analise_programa(buffer: &mut Buffer) -> Expressao {
    let tok = proximo_token(buffer);

    if tok.tipo != TipoToken::Print {
        panic!("Token nao esperado: {:?} na linha {}", tok, tok.linha)
    }

    analise_expressao(buffer)
}

fn analise_expressao(buffer: &mut Buffer) -> Expressao {
    let mut tok = proximo_token(buffer);

    match tok.tipo {
        TipoToken::Inteiro(n) => Expressao::Constante(n),
        TipoToken::AbreParentese => {
            let op1 = analise_expressao(buffer);
            tok = proximo_token(buffer);
            let operador = match tok.tipo {
                TipoToken::Soma => Operador::Soma,
                TipoToken::Asterisco => Operador::Mult,
                _ => panic!("Operador esperado, encontrado: {:?} na linha {}", tok, tok.linha)
            };
            let op2 = analise_expressao(buffer);
            tok = proximo_token(buffer);
            if tok.tipo != TipoToken::FechaParentese {
                panic!("Parentese direito esperado, encontrado: {:?} na linha {}", tok, tok.linha);
            }

            match operador {
                Operador::Soma => Expressao::Soma(Box::new(op1), Box::new(op2)),
                Operador::Mult => Expressao::Mult(Box::new(op1), Box::new(op2))
            }
        }
        _ => panic!("Token nao esperado no comeco de expressao: {:?} na linha {}", tok, tok.linha)
    }
}


// testes
#[test]
fn teste_sintatico_exp_constante() {
    let mut buffer = Buffer::cria_com_string("print 42");
    let arv = analise_programa(&mut buffer);
    assert_eq!(arv, Expressao::Constante(42));
}

#[test]
fn teste_sintatico_exp_soma_simples() {
    let mut buffer = Buffer::cria_com_string("print (4 + 7)");
    let arv = analise_programa(&mut buffer);
    assert_eq!(arv, Expressao::Soma(Box::new(Expressao::Constante(4)),
                                    Box::new(Expressao::Constante(7))));
}

#[test]
fn teste_sintatico_exp_dois_operadores() {
    let mut buffer = Buffer::cria_com_string("print (4 + (39 * 6))");
    let arv = analise_programa(&mut buffer);
    assert_eq!(arv,
               Expressao::Soma(Box::new(Expressao::Constante(4)),
                               Box::new(Expressao::Mult(Box::new(Expressao::Constante(39)),
                                                        Box::new(Expressao::Constante(6))))));
}
