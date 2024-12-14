/* 1 - Declarando Variáveis e Imutabilidade
Declare duas variáveis, a e b, sendo a mutável e b imutável.
Atribua valores iniciais a a e b e, em seguida, tente modificar ambos os valores.
Observe o que acontece ao tentar alterar a variável imutável.*/

/*fn main() {
    let a_imutavel = 10;
    let mut b_mutavel = 10;
    a_imutavel = 15;
    b_mutavel = 15;

    println!("{}", a_imutavel);
    println!("{}", b_mutavel);
}*/

/*2. Variable Shadowing
Declare uma variável x e atribua o valor 10.
Redefina x como uma string com o valor "dez".
Imprima o valor de x após cada redefinição para entender como o shadowing funciona.*/

/*fn main() {
    let x = 10;
    println!("{}", x);
    let x = "dez";
    println!("{}", x);
}*/

/*3. Escopos e Lifetime
Declare uma variável y no escopo principal.
Dentro de um escopo interno (usando {}), redefina y adicionando um valor a ele.
Imprima o valor de y dentro e fora do escopo interno para observar a diferença.*/

/*fn main() {
    let y = 10; {
        let y = 15;
        println!("inside: {}", y);
    }
    println!("out: {}", y);
}*/

/*4. Usando Constantes
Crie uma constante chamada DAYS_IN_WEEK com o valor 7.
Declare uma variável que armazene a quantidade de horas em uma semana usando a constante e o valor de 24 para horas em um dia.
Imprima o resultado.*/

/*fn main() {
    const DAYS_IN_WEEK: u32 = 7;
    let var_hours = 24;
    let var_hours_in_week = DAYS_IN_WEEK * var_hours;
    println!("the week has {} days and one day has {} hours and one week has {} hours", DAYS_IN_WEEK, var_hours, var_hours_in_week);
}*/

/*5. Calculadora Simples
Crie uma função chamada calculate_total_seconds que receba um número de minutos como entrada e retorne o total em segundos usando a constante SECONDS_IN_MINUTE.
Chame a função no main, passando diferentes valores e imprimindo os resultados.*/

/*const SECONDS_IN_MINUTE: i32 = 60;
fn calculate_total_seconds(minutes: i32) -> i32 {
    minutes * SECONDS_IN_MINUTE
}
fn main() {
    let minute1 = 5;
    let minute2 = 10;
    let minute3 = 15;

    let total_seconds1 = calculate_total_seconds(minute1);
    let total_seconds2 = calculate_total_seconds(minute2);
    let total_seconds3 = calculate_total_seconds(minute3);

    println!("{} minutes are {} seconds", minute1, total_seconds1);
    println!("{} minutes are {} seconds", minute2, total_seconds2);
    println!("{} minutes are {} seconds", minute3, total_seconds3);
}*/

/*6. Praticando Drop
Crie uma variável que armazene uma String.
Dentro de um escopo interno, defina outra String que é uma concatenação da primeira com mais texto.
Imprima a segunda string dentro do escopo e tente acessá-la fora do escopo. Observe o que acontece.*/

/*fn main() {
    let string1 = "lemonade"; {
        let string2 = "lemonade is nice";
        println!("{}", string1);
    }
}*/

/*7. Criando Placeholders
Declare variáveis para um nome, idade e salário.
Use println! para imprimir uma frase formatada, como:
"O funcionário João tem 30 anos e recebe um salário de 2500.50 reais por mês."*/

fn main() {
    let name = "Pedro";
    let age = 20;
    let wage = 655;
    println!("O funcionário {} tem {} anos e recebe um salário de {} reais por mês", name, age, wage);
}

/*8. Praticando Mutabilidade
Crie uma variável contador inicializada com 0 e marque-a como mutável.
Use um loop para incrementar o valor de contador até 5.
Imprima o valor de contador a cada iteração.*/
