/* Exercício: Trabalhando com Tipos de Dados em Rust
Objetivo: 
*/

/* Tipos escalares:

Declare e inicialize uma variável idade do tipo inteiro u8 com o valor 25.
Declare e inicialize uma variável altura do tipo flutuante f32 com o valor 1.75.
Declare uma variável booleana maior_de_idade que será true se a idade for maior ou igual a 18.
Imprima os valores dessas variáveis.

fn main() {
    let idade: u8 = 25;
    let altura: f32 = 1.75;
    let mut maior_idade: bool = false;

    if idade >= 18 {
        maior_idade = true;
    }
    println!("idade: {:?} altura: {:?} maior_idade: {:?}", idade, altura, maior_idade);
} */   

/* Conversão e literais:

Declare uma variável peso no formato hexadecimal representando o número decimal 70 (0x46) e converta-o para decimal.
Declare uma variável populacao no formato binário representando o número 8 e converta para decimal.
Imprima os valores convertidos.

fn main() {
    let hexa_decimal = 0x46;
    let populacao_binary = 0b1000;
    
    println!("{:?}",hexa_decimal);
    println!("{:?}",populacao_binary);
} */

/* Trabalhando com char e Unicode:

Crie uma variável inicial que receba a inicial do seu nome e uma variável emoji com o emoji '🎉'.
Imprima o inicial e o emoji.
Tuplas:

fn main() {
    let inicial: char = 'P';
    let emoji: char = '🎉';

    println!("inicial: {} emoji: {}", inicial, emoji);
} */

/* Declare uma tupla dados_pessoais contendo os valores (idade, altura, maior_de_idade).
Extraia os valores da tupla para variáveis separadas usando pattern matching.
Imprima cada valor.
Arrays:

fn main() {
    let dados_pessoais: (i32, f64, i32) = (20, 1.75, 18);
    let (idade, altura, maior_de_idade) = dados_pessoais;

    println!("idade: {:?}", idade);
    println!("altura: {:?}", altura);
    println!("maior_de_idade: {:?}", maior_de_idade);
} */

/* Crie um array notas do tipo f32 com os valores [7.5, 8.0, 9.5].
Substitua o segundo elemento do array por 10.0.
Imprima o array modificado e a fatia contendo os dois primeiros elementos.

fn main() {
    let mut notas: [f32;3] = [7.5, 8.0, 9.5];
    notas[1] = 10.0;

    println!("notas: {:?}", &notas[..2]);
} */