// prova rust 28/05

//////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////// Nível 1 /////////////////////////////////////////////

use std::collections::LinkedList;

fn main() {
    // Aqui criei uma lista encadeada vazia
    let mut lista: LinkedList<i32> = LinkedList::new();



    // Colocando o número no final da lista
    lista.push_back(10);
    println!("Após inserir o número 10: {:?}", lista);

    lista.push_back(20);
    println!("Após inserir o número 20: {:?}", lista);

    lista.push_back(30);
    println!("Após inserir o número 30: {:?}", lista);



    // Colocando o número no início da lista
    lista.push_front(5);
    println!("Depois de colocar o 5 no início: {:?}", lista);



    // Remove o primeiro elemento
    if let Some(removido) = lista.pop_front() {
        println!("Número removido do início: {}", removido);
    }
    println!("lista depois de remover do início: {:?}", lista);


    // Remove o último elemento
    if let Some(removido) = lista.pop_back() {
        println!("Removido do final: {}", removido);
    }
    println!("lista depois de remover do final: {:?}", lista);
}


//////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// Nível 2 (struct e Box)////////////////////////////////////////


// Nível 2 (struct e Box)

// Defini o nó da lista com um valor inteiro e um ponteiro para o próximo nó (ou None se for o último)
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

// Defini a lista encadeada que guarda o início da lista, que pode estar vazia (None)
pub struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    // Criando uma lista vazia 
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    // Inserindo um valor no início da lista
    pub fn push_front(&mut self, value: i32) {

        // Criando um novo nó que aponta para o atual head e atualiza o head para esse novo nó
        self.head = Some(Box::new(Node { value, next: self.head.take() }));

    }

    // Inserindo um valor no final da lista
    pub fn push_back(&mut self, value: i32) {

        // Criando um novo nó com next = None (último nó)
        let mut new_node = Box::new(Node { value, next: None });

        match self.head.as_mut() {
            Some(mut node) => {
                // Percorrendo a lista até encontrar o último nó 
                while let Some(ref mut next) = node.next {
                    node = next;  // Avança para o próximo nó
                }
                // Aponta o next do último nó para o novo nó criado
                node.next = Some(new_node);
            }
            None => 
                // Se a lista está vazia, o novo nó vira o head
                self.head = Some(new_node),
        }
    }

    // Remove e retorna o valor do primeiro elemento da lista, se tiver
    pub fn pop_front(&mut self) -> Option<i32> {
        // Remove o head atual, substitui pelo próximo e retorna o valor removido
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    // Exibe todos os elementos da lista no formato "valor -> valor -> None"
    pub fn display(&self) {
        let mut current = &self.head; // Começa pelo head
        print!("Lista: ");
        // Enquanto existir um nó atual
        while let Some(node) = current {
            print!("{} -> ", node.value); // Imprime o valor
            current = &node.next;          // Vai para o próximo nó
        }
        println!("None"); // Indica o fim da lista
    }
}

fn main() {
    let mut lista = LinkedList::new(); 
    lista.display();                  

    //colocando os valores
    lista.push_front(10);  // 10 no início 
    lista.push_back(20);   // 20 no fim 
    lista.push_back(30);   // 30 no fim
    lista.push_front(5);   // 5 no início 
    lista.display();       // Exibe a lista atual

    // Remove o primeiro elemento e, se houver, imprime o valor removido
    if let Some(v) = lista.pop_front() {
        println!("Removido: {}", v);
    }

     // Exibe a lista após remoção
    lista.display(); 
}
