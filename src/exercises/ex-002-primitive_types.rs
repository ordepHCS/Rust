/* Exercício: Trabalhando com Tipos de Dados em Rust
Objetivo:
Criar um programa que:

Utilize diferentes tipos escalares e compostos.
Realize operações e manipulações nos tipos.
Imprima os resultados no console.
Enunciado:
Crie um programa em Rust que execute as seguintes etapas: */

/* Tipos escalares:

Declare e inicialize uma variável idade do tipo inteiro u8 com o valor 25.
Declare e inicialize uma variável altura do tipo flutuante f32 com o valor 1.75.
Declare uma variável booleana maior_de_idade que será true se a idade for maior ou igual a 18.
Imprima os valores dessas variáveis. */

fn main() {
    let idade: u8 = 25;
    let altura: f32 = 1.75;
    let mut maior_idade: bool = false;

    if idade >= 18 {
        maior_idade = true;
    }
    println!("idade: {:?} altura: {:?} maior_idade: {:?}", idade, altura, maior_idade);
}   
