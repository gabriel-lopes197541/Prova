use std::collections::LinkedList;
// Nível 1
fn main() {
    let mut lista = LinkedList::new();
    // Adiciona ao início
    lista.push_front(10);
    println!("Inserido no início: {:?}", lista);

    lista.push_back(15);
    println!("Inserido no meio: {:?}", lista);


    lista.push_back(20);
    println!("Inserido no final: {:?}", lista);

    // Remove no início
    let removido_front = lista.pop_front();
    println!("Lista após pop_front(): {:?}", lista);
    println!("Elemento removido: {:?}", removido_front);

    // Remove do final
    let removido_back = lista.pop_back();
    println!("Lista após pop_back(): {:?}", lista);
    println!("Elemento removido: {:?}", removido_back);
}
