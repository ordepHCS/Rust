/*declarando variaveis e constantes em Rust!
rust precisa ter um escopo definido para declarar variaveis
fn main() { escopo 
    let _total = 30; "_" para o compilador entender que a var nao sera utilizada"
    let mut total = 30; toda variavel em Rust por padrao e imutavel para se tornar mutavel usar o comando "mut"

    println!("{}", total); place holder em rust: {} - para inteiros, floats, strings etc..

    total = 40;

    println!("{}", total);

    let total = "quarenta"; variable shadowing: redefinindo uma variavel

    println!("{}", total);

    life time a partir do escopo
    escopo externo
    let total2 = 30; { inicio
        escopo interno
        let total2 = total2 + 20;
        println!("interno = {}", total2);
    } fim 
    println!("externo = {}", total2);
} fim
Drop: quando um objeto sai do escopo a funcao Drop e chamada automaticamente 
liberando qualquer recurso associado a ele ex: memoria, arquivos abertos, etc..*/

// constantes em Rust
// constantes em rusts podem ser definidas dentro ou fora de uma funcao ou dentro de um contexto do interior de uma função
const SECONDS_IN_MINUTE: u32 = 60; // constantes em Rust sao imutaveis
fn main() {
    // constantes declaradas dentro do escopo 
    const _MINUTES_IN_HOUR: u32 = 60;
    const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE;

    let total = 30;
    let total_in_seconds = total * SECONDS_IN_HOUR;
    println!("{}", total);
    println!("{}",total_in_seconds);
}