# minicalc-rs

Interpretador MiniCalc escrito em Rust.

## Sobre a linguagem

MiniCalc é uma linguagem simples para cálculo de expressões aritméticas.
O interpretador para a linguagem será desenvolvido em estágios, começando
com a versão mais simples e gradativamente adicionando novos
recursos.

## Estágio 1

Programas são da forma

```
print E
```

sendo E uma expressão. As expressões podem ser:

```
E ::= (E + E) | (E * E)
E ::= <numero>
```

`<numero>` é um número inteiro.

Exemplo 1:
```
print 2
```

Exemplo 2:
```
print (4 + 5)
```

Exemplo 3:
```
print (4 + (2 * 3))
```

## Estágio 2

Mais operações.


## Estágio 3

Inlcui a possibilidade de declarar e utilizar variáveis nas expressões.

Programas são da forma

```
var <ident> = E;
var <ident> = E;
...
print E
```

`<ident>` é um identificador. Um identificador é uma sequência de caracteres,
começando por uma letra, que pode incluir letras, dígitos e o caractere
sublinhado (underline).

As expressões podem ser:

```
E ::= (E + E) | (E * E)
E ::= <numero> | <ident>
```

Exemplo 1:

```
var x = 42;
var y = 11;
print (x + (y * 31))
```

Exemplo 2:

```
var x = (42 + 78);
var y = (x * 278);
print (x + (y * 31))
```


## Estágio 4

Precedência de operadores.
