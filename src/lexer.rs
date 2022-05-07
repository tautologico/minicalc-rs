#[derive(Debug, PartialEq)]
pub enum TipoToken {
    AbreParentese,
    FechaParentese,
    Soma,
    Asterisco,
    Inteiro(i64),
    Print,
    Eof
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub tipo: TipoToken,
    linha: usize,
}

impl Token {
    pub fn simbolo(tipo: TipoToken, linha: usize) -> Token {
        Token { tipo: tipo, linha: linha }
    }

    pub fn eof(linha: usize) -> Token {
        Token { tipo: TipoToken::Eof, linha: linha }
    }
}

pub struct Buffer {
    entrada: String,
    entrada_caracteres: Vec<char>,
    pos: usize,
    pub linha: usize
}

impl Buffer {
    pub fn cria_com_string(s: &str) -> Buffer {
        Buffer {
            entrada: s.to_string(),
            entrada_caracteres: s.to_string().chars().collect(),
            pos: 0,
            linha: 1
        }
    }

    pub fn no_final(&self) -> bool {
        self.pos >= self.entrada_caracteres.len()
    }

    pub fn proximo_caractere(&mut self) -> Option<char> {
        if self.no_final() {
            None
        } else {
            let resultado = self.entrada_caracteres[self.pos];
            self.pos += 1;
            Some(resultado)
        }
    }

    pub fn posicao_anterior(&mut self) {
        self.pos -= 1;
    }

    /// Avanca a posicao atual do buffer enquanto o caractere atual for
    /// espaco em branco (espaco, tab, nova linha)
    pub fn avanca_espaco(&mut self) {
        while !self.no_final() && self.entrada_caracteres[self.pos].is_whitespace() {
            if self.entrada_caracteres[self.pos] == '\n' {
                self.linha += 1;
            }
            self.pos += 1;
        }
    }
}

pub fn proximo_token(buffer: &mut Buffer) -> Token {
    if buffer.no_final() {
        return Token::eof(buffer.linha);
    }

    buffer.avanca_espaco();

    // examina o proximo caractere da entrada
    match buffer.proximo_caractere() {
        Some('(') => Token::simbolo(TipoToken::AbreParentese, buffer.linha),
        Some(')') => Token::simbolo(TipoToken::FechaParentese, buffer.linha),
        Some('+') => Token::simbolo(TipoToken::Soma, buffer.linha),
        Some('*') => Token::simbolo(TipoToken::Asterisco, buffer.linha),
        Some(c) if c.is_digit(10) => token_numero(buffer, c),
        Some(c) if c.is_alphabetic() => token_palavra_chave(buffer, c),
        None => Token::eof(buffer.linha),
        Some(c) => panic!("caractere nao esperado: {} na linha {}", c, buffer.linha)
    }
}

pub fn token_numero(buffer: &mut Buffer, c: char) -> Token {
    // acumular os digitos em string, entao converter p/ inteiro
    let mut digitos : Vec<char> = vec!(c);

    let mut prox = buffer.proximo_caractere();

    while prox.is_some() && prox.expect("q?").is_digit(10) {
        digitos.push(prox.expect("q?"));
        prox = buffer.proximo_caractere();
    }

    // retornar o ultimo caractere lido para o buffer
    if prox.is_some() {
        buffer.posicao_anterior();
    }

    let s : String = digitos.iter().collect();

    let valor = i64::from_str_radix(&s, 10).
        expect("token_numero: Erro convertendo string p/ numero");

    Token { tipo: TipoToken::Inteiro(valor), linha: buffer.linha }
}

pub fn token_palavra_chave(buffer: &mut Buffer, c: char) -> Token {
    let mut letras : Vec<char> = vec!(c);
    let mut prox = buffer.proximo_caractere();

    while prox.is_some() && prox.expect("q?").is_alphabetic() {
        letras.push(prox.expect("q?"));
        prox = buffer.proximo_caractere();
    }

    if prox.is_some() {
        buffer.posicao_anterior();
    }

    let s : String = letras.iter().collect();

    if s != "print" {
        panic!("palavra-chave nao reconhecida: {} na linha {}", s, buffer.linha);
    }

    Token { tipo: TipoToken::Print, linha: buffer.linha }
}

#[test]
fn teste1_lexer() {
    let teste1 = "( + * print   \n\n43257)   ";
    let mut buffer1 = Buffer::cria_com_string(teste1);

    assert_eq!(proximo_token(&mut buffer1), Token { tipo: TipoToken::AbreParentese, linha: 1 });
    assert_eq!(proximo_token(&mut buffer1), Token { tipo: TipoToken::Soma, linha: 1 });
    assert_eq!(proximo_token(&mut buffer1), Token { tipo: TipoToken::Asterisco, linha: 1 });
    assert_eq!(proximo_token(&mut buffer1), Token { tipo: TipoToken::Print, linha: 1 });
    assert_eq!(proximo_token(&mut buffer1), Token { tipo: TipoToken::Inteiro(43257), linha: 3 });
    assert_eq!(proximo_token(&mut buffer1), Token { tipo: TipoToken::FechaParentese, linha: 3 });
    assert_eq!(proximo_token(&mut buffer1), Token { tipo: TipoToken::Eof, linha: 3 });
}
