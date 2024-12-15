/* Crie um array notas do tipo f32 com os valores [7.5, 8.0, 9.5].
Substitua o segundo elemento do array por 10.0.
Imprima o array modificado e a fatia contendo os dois primeiros elementos. */

fn main() {
    let mut notas: [f32;3] = [7.5, 8.0, 9.5];
    notas[1] = 10.0;

    println!("notas: {:?}", &notas[..2]);
}