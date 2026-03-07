use std::cmp::max;

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug, Default)]
struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    fn new() -> Self {
        Self { root: None }
    }

    fn insert(&mut self, value: i32) {
        Self::insert_node(&mut self.root, value);
    }

    fn insert_node(current: &mut Option<Box<Node>>, value: i32) {
        match current {
            Some(node) => {
                if value < node.value {
                    Self::insert_node(&mut node.left, value);
                } else {
                    Self::insert_node(&mut node.right, value);
                }
            }
            None => {
                *current = Some(Box::new(Node::new(value)));
            }
        }
    }

    fn contains(&self, target: i32) -> bool {
        Self::contains_node(&self.root, target)
    }

    fn contains_node(current: &Option<Box<Node>>, target: i32) -> bool {
        match current {
            Some(node) if target == node.value => true,
            Some(node) if target < node.value => Self::contains_node(&node.left, target),
            Some(node) => Self::contains_node(&node.right, target),
            None => false,
        }
    }

    fn inorder(&self) -> Vec<i32> {
        let mut values = Vec::new();
        Self::inorder_node(&self.root, &mut values);
        values
    }

    fn inorder_node(current: &Option<Box<Node>>, values: &mut Vec<i32>) {
        if let Some(node) = current {
            Self::inorder_node(&node.left, values);
            values.push(node.value);
            Self::inorder_node(&node.right, values);
        }
    }

    fn height(&self) -> usize {
        Self::height_node(&self.root)
    }

    fn height_node(current: &Option<Box<Node>>) -> usize {
        match current {
            Some(node) => 1 + max(Self::height_node(&node.left), Self::height_node(&node.right)),
            None => 0,
        }
    }
}

fn inspect_scores(scores: &[i32]) {
    println!("Pontuacoes recebidas por referencia: {:?}", scores);
    println!("Quantidade de pontuacoes: {}", scores.len());
}

fn main() {
    println!("Demo: BST em Rust com ownership e Box<Node>");
    println!("-------------------------------------------");

    let scores = vec![50, 30, 70, 20, 40, 60, 80, 65, 10, 35];
    inspect_scores(&scores);

    let mut tree = BinarySearchTree::new();
    for score in &scores {
        tree.insert(*score);
    }

    println!("\nBST criada.");
    println!("In-order (ordenado): {:?}", tree.inorder());
    println!("Altura da arvore: {}", tree.height());

    for target in [65, 99, 20] {
        println!("Contem {}? {}", target, tree.contains(target));
    }

    let owner_message = String::from("Ownership evita use-after-free.");
    print_message(&owner_message);
    println!("Mensagem ainda existe porque foi emprestada: {}", owner_message);
}

fn print_message(message: &str) {
    println!("\nBorrowing imutavel: {}", message);
}
