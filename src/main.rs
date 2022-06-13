pub mod lexer;
pub mod parser;

use lexer::Buffer;
use parser::Expressao;

fn main() {
    let teste1 = "print (4 + (39 * 6))";

    let mut buffer1 = Buffer::cria_com_string(teste1);

    println!("Interpretador Minicalc");
    println!("Teste do analisador sintatico");

    let arvore = parser::analise_programa(&mut buffer1);

    println!("Arvore sintatica: {:?}", arvore);

    println!("### Valor da expressao: {}", avalia_expressao(&arvore));

}

fn avalia_expressao(e: &Expressao) -> i64 {
    match e {
        Expressao::Constante(n) => *n,
        Expressao::Soma(op1, op2) => avalia_expressao(op1) + avalia_expressao(op2),
        Expressao::Mult(op1, op2) => avalia_expressao(op1) * avalia_expressao(op2)
    }
}


#[test]
fn teste_aval_exp_constante() {
    let mut buffer = Buffer::cria_com_string("print 42");
    let e = parser::analise_programa(&mut buffer);
    assert_eq!(avalia_expressao(&e), 42);
}

fn teste_aval_exp_dois_operadores() {
    let mut buffer = Buffer::cria_com_string("print (4 + (39 * 6))");
    let e = parser::analise_programa(&mut buffer);
    assert_eq!(avalia_expressao(&e), 238);
}
