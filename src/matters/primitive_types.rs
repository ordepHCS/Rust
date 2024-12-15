/* # Escalares (scalar types) 

- Representam um unico valor contido dentro de uma escala conhecida
- Permitem a comparação direta de valores

-  Tipos:

- inteiro(integer - int) ex: '5'
- flutuante(floating point - float) ex: '42.1'
- booleano(bool) ex: 'true', 'false'
- caractere(char) ex: 'a', emojis '🤓' ou caracteres especiais como '愛'

   # Compostos (Compund type)

- Servem para agregar multiplos valores

-  Tipos:

- tupla(tuple) ex: '(5, true, 42.1, 'a')'
- matriz(array) ex: '[1, 2, 3, 4, 5]'

   # Inteiros

   | bits | signed | unsigned |
   |------|--------|----------|
   | 8    | i8     | u8       |
   | 16   | i16    | u16      |
   | 32   | i32    | u32      |
   | 64   | i64    | u64      |
   | 128  | i128   | u128     |
   | arch | isize  | usize    |

   # signed

   range: -(2ⁿ⁻¹) ate 2ⁿ⁻¹ - 1
   i8: -128 ate 127 [-(2⁷) ate 2⁷ - 1]

   # unsigned

   range: 0 ate 2ⁿ - 1
   u8 0 ate 255 [0 ate 2⁸ - 1]

*/

/* overflow:
fn main() {
   let x: u8 = 5;
   let y: u8 = x - 20; // seria negativo resultando em overflow pos u8 nao suporta valores negativos
}*/

/* literais
fn main() {
   let x = 5;
   let y = 199_456_988; /* dividir valores com "_" para melhor visualização
          "199456988"*/
   let hexa_decimal = 0xff; /* prefixo 0x indica que o numero esta representado no sistema hexadecimal (base 16)
                 valor de ff equivale a 15 x 16¹ + 15 x 16⁰ = 255 em decimal (base 10) portanto hexa_decimal recebe o valor 255*/

   let octal = 0o77; /* prefixo 0o indica que o numero esta no formto octal (base 8) em octal os numeros utilizam apenas os digitos de 0 a 7
                 valor de 0o77: no sistema octal cada posicao equivale a uma potencia de 8:
                                                                                            0o77 = (7 x 8¹) + (7 x 8⁰) = 56 + 7 = 63 portanto 0o77 representa o numero 63 em decimal */
   
   let binary = 0b1111_0000; /* prefixo 0b indica que o numero esta no formato binario em binario os numeros utilizam apenas os digitos 0 e 1
                        valor de 0b1111_0000: cada posicao no binario representa uma potencia  de 2 (da direita para a esquerda):
                        0b1111_0000 = (1 x 2⁷) + (1 x 2⁶) + (1 x 2⁵) + (1 x 2⁴) + (0 x 2³) + (0 x 2²) + (0 x 2¹) + (0 x 2⁰)
                        isso resulta em: (128 + 64 + 32 + 16 + 0 + 0 + 0 + 0) = 240 em decimal */
   
   let bytes = b'A'; /* prefixo b indica que o literal e do tipo byte e nao um char unicode
                        o resultado sera um valor do tipo u8(8bits) representando o codigo ASCII do char
                        valor de 'A': no sistema ASCII o char 'A' tem o codigo decimal 65
                        assim b'A' atribui o valor numerico 65(em decimal) a variavel bytes */
} */

/* # Numeros floats

fn main() {
   let x: f64 = 42.1; 
} */

/* # Booleano

fn main() {
   let x = true;
   let y = false;
   let z: bool = true;
} */

/* # Char and Unicode

fn main() {
   let letra: char = 'a'; // possibilidade para 4 bytes dentro da tabela unicode 
} */

/* # Tupla

fn main() {
   let numbers: (i32, i32, i32) = (1, 2, 3.5);
   println!("{:?}", numbers);
   println!("{:?}", numbers.0); .0 pega o elemento exato da tupla (posicao 0: 1)
} */

/* # Pattern match em tuplas 

fn main() {
   let numbers = (1, 2, 3);
   // antes: 1 2 3
   let(a, b, c) = numbers;
   // agora a b c
   println!("{:?}", a);
   println!("{:?}", b);
   println!("{:?}", c);

   // tupla mutavel
   let mut numberss: (i32, i32, i32) = (1, 2, 3);
   numberss.0 = 50;

   println!("{:?}", numberss);

   numberss.0 = 1;
   numberss.1 = 2;
   numberss.2 = 3;

   println!("{:?}", numberss.0);
   println!("{:?}", numberss.1);
   println!("{:?}", numberss.2);
} */

/* # Array

fn main() {
   let numbers: [i32;3] = [1, 2, 3]; 
   println!("{:?}", numbers[0]);
   println!("{:?}", numbers[1]);
   println!("{:?}", numbers[2]);

   // array mutavel
   let mut numberss: [i32;3] = [1, 2, 3];
   numberss[0] = 3;
   println!("{:?}", numberss[0]);
} */

/* # Slicing

fn main () {
   let numbers: [f64;3] = [1.1, 2.0, 3.3];
   println!("{:?}", &numbers[1..3]);
   println!("{:?}", &numbers[..2]);
} */
