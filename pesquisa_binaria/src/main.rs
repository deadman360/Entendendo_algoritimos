use std::time::Instant;

fn main() {
    let mut vetor: Vec<i64> = (1..1_000_000_00).collect();
    
    let start: Instant = Instant::now();
    
    pesquisa_binaria(999_999_99, &vetor);
    
    let duration = start.elapsed();

    let start2: Instant = Instant::now();

    pesquisa_simples(999_999_99, &mut vetor);

    let duration_simples = start2.elapsed();
    println!("Duração pesquisa binaria = {:?}\n duração pesquisa simples = {:?}", duration, duration_simples)

}
fn pesquisa_binaria(target: i64, lista: &Vec<i64>) -> Option<i64> {
    let mut inicio: usize = 0;
    let mut fim: usize = lista.len();
    /*tentei fazer isso atraves de recursao porem deu stack overflow pelo  tanto de funções diretamente na stack
    loops não são tao ruins quando o numero de funções na stack pode ser muito grande
    alias fazendo atraves de recursâo o tempo de execução foi maior quando eu consegui fazer que ele nao desse stack overflow
    usando o metodo splitoff  */
    while inicio >= fim{

        let meio: usize = (inicio+fim)/2;
        if target == lista[meio]{                               
             return Some(lista[meio]);
        }else if target < lista[meio] {
            fim = meio-1;
        }else if target > lista[meio]  {
            inicio = meio+1;
        }
    }
    None
}
fn pesquisa_simples(target: i64, lista: &mut Vec<i64>) {
    for i in lista {
        if *i == target{
            println!("Encontrado! {}", i)
        }
        
    }
}