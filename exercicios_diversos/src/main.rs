fn main() {
    let lista: Vec<u8> = (1..=254).collect();
    println!("Numeros pares da lista = {:?}", numeros_pares(lista));
    println!("A soma dos digitos do numero {} da {}", 32, soma_digitos(32));

}
/*função que recebe lista com numeros e retorna somente os pares */
fn numeros_pares(lista: Vec<u8>) -> Vec<u8> {
    let mut lista_retorno: Vec<u8> = Vec::with_capacity((lista.len()/2)+1);
    for v in lista {
        if v % 2 == 0 {
            lista_retorno.push(v);
        }else{
            continue;
        }
    }
    lista_retorno
}
/*função que recebe um inteiro e retorna a soma de seus digitos */
fn soma_digitos(inteiro: i32) -> u32{
    let mut contador: u32 = 0;
    for v in inteiro.to_string().chars(){
        let v_decimal: Option<u32> = v.to_digit(10);
        match v_decimal{
            Some(valor) => contador += valor,
            None => continue
        }
    }
    contador
}

