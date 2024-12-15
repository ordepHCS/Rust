/* Declare uma tupla dados_pessoais contendo os valores (idade, altura, maior_de_idade).
Extraia os valores da tupla para variáveis separadas usando pattern matching.
Imprima cada valor.
Arrays: */

fn main() {
    let dados_pessoais: (i32, f64, i32) = (20, 1.75, 18);
    let (idade, altura, maior_de_idade) = dados_pessoais;

    println!("idade: {:?}", idade);
    println!("altura: {:?}", altura);
    println!("maior_de_idade: {:?}", maior_de_idade);
}
