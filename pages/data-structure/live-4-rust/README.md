# Live 4 - Rust e Gerenciamento de Memoria

Este exemplo acompanha a aula 4 de `data-structures` e mostra uma **Binary Search Tree em Rust** usando:

- `Option<Box<Node>>`
- ownership
- borrowing
- travessia in-order
- busca e altura da arvore

O arquivo principal e:

`main.rs`

## 1. Pre-requisito

Voce precisa ter Rust instalado.

Se ainda nao tiver:

```bash
curl https://sh.rustup.rs -sSf | sh
```

Depois feche e abra o terminal, ou rode:

```bash
source "$HOME/.cargo/env"
```

## 2. Rodando no terminal com `rustc`

Entre na pasta:

```bash
cd pages/data-structure/live-4-rust
```

Compile:

```bash
rustc main.rs -o bst_memory_demo
```

Execute:

```bash
./bst_memory_demo
```

## 3. Rodando no VS Code

1. Abra a pasta do repositorio no VS Code.
2. Navegue ate `pages/data-structure/live-4-rust/main.rs`.
3. Instale a extensao `rust-analyzer` se ainda nao tiver.
4. Abra o terminal integrado.
5. Rode:

```bash
cd pages/data-structure/live-4-rust
rustc main.rs -o bst_memory_demo
./bst_memory_demo
```

## 4. O que esperar da saida

O programa vai:

- imprimir as pontuacoes iniciais
- montar a BST
- mostrar a travessia in-order ordenada
- mostrar a altura da arvore
- testar buscas
- demonstrar borrowing com uma string emprestada

## 5. Ideias para expandir

- implementar remocao
- adicionar minimo e maximo
- criar versao AVL
- transformar em um projeto Cargo

