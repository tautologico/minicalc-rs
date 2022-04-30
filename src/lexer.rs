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
    linha: usize
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

    pub fn proximo_caractere(&mut self) -> char {
        let resultado = self.entrada_caracteres[self.pos];
        self.pos += 1;
        resultado
    }

    pub fn posicao_anterior(&mut self) {
        self.pos -= 1;
    }
}

pub fn proximo_token(buffer: &mut Buffer) -> Token {
    if buffer.no_final() {
        return Token::eof(buffer.linha);
    }

    // TODO consome espaco em branco e atualiza campo linha
    // TODO tratamento de erros lexicos

    // examina o proximo caractere da entrada
    match buffer.proximo_caractere() {
        '(' => Token::simbolo(TipoToken::AbreParentese, buffer.linha),
        ')' => Token::simbolo(TipoToken::FechaParentese, buffer.linha),
        '+' => Token::simbolo(TipoToken::Soma, buffer.linha),
        '*' => Token::simbolo(TipoToken::Asterisco, buffer.linha),
        c if c.is_digit(10) => token_numero(buffer, c),
        c if c.is_alphabetic() => token_palavra_chave(buffer, c),
        _ => panic!("caractere nao esperado")
    }
}

pub fn token_numero(buffer: &mut Buffer, c: char) -> Token {
    // acumular os digitos em string, entao converter p/ inteiro
    let mut digitos : Vec<char> = vec!(c);

    let mut prox = buffer.proximo_caractere();

    while prox.is_digit(10) {
        digitos.push(prox);
        prox = buffer.proximo_caractere();
    }

    // retornar o ultimo caractere lido para o buffer
    buffer.posicao_anterior();

    let s : String = digitos.iter().collect();

    let valor = i64::from_str_radix(&s, 10).
        expect("token_numero: Erro convertendo string p/ numero");

    Token { tipo: TipoToken::Inteiro(valor), linha: buffer.linha }
}

pub fn token_palavra_chave(buffer: &mut Buffer, c: char) -> Token {
    let mut letras : Vec<char> = vec!(c);
    let mut prox = buffer.proximo_caractere();

    while prox.is_alphabetic() {
        letras.push(prox);
        prox = buffer.proximo_caractere();
    }

    let s : String = letras.iter().collect();

    if s != "print" {
        panic!("palavra-chave nao reconhecida");
    }

    Token { tipo: TipoToken::Print, linha: buffer.linha }
}
