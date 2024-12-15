/* Conversão e literais:

Declare uma variável peso no formato hexadecimal representando o número decimal 70 (0x46) e converta-o para decimal.
Declare uma variável populacao no formato binário representando o número 8 e converta para decimal.
Imprima os valores convertidos. */

fn main() {
    let hexa_decimal = 0x46;
    let populacao_binary = 0b1000;
    
    println!("{:?}",hexa_decimal);
    println!("{:?}",populacao_binary);
}
