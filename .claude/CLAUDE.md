# Lives - Instrucoes Claude Code

## Sobre o Projeto

Repositorio com materiais de lives em HTML e notebooks Jupyter para disciplinas como `applied-math` e `data-structure`.

O padrao atual do projeto nao usa geradores externos de slide. As lives sao feitas manualmente em HTML com `Reveal.js`, e os exemplos praticos ficam em arquivos `.ipynb`.

## Estrutura Relevante

```text
/
├── index.html
├── pages/
│   ├── applied-math/
│   │   ├── main.html
│   │   ├── live-1.html
│   │   ├── live-1.ipynb
│   │   └── ...
│   └── data-structure/
│       ├── main.html
│       ├── live-1.html
│       ├── live-1.ipynb
│       └── ...
└── .claude/
    ├── CLAUDE.md
    ├── revealjs-patterns.md
    └── notebook-patterns.md
```

## Regras Gerais

1. Seguir o padrao existente do repositorio: `main.html` por disciplina e arquivos `live-N.html` / `live-N.ipynb`.
2. Nao trocar o stack local por outro sistema de slides.
3. Slides devem usar `Reveal.js` com HTML estatico.
4. Cada live nova deve vir acompanhada de um notebook correspondente quando houver exemplos praticos.
5. Ao criar uma live, atualizar a descricao do card da disciplina em `pages/<disciplina>/main.html`.

## Guias de Referencia

- `revealjs-patterns.md`: padroes para lives em HTML com `Reveal.js`
- `notebook-patterns.md`: padroes para notebooks das aulas

## Fluxo Recomendado

1. Inspecionar a disciplina e o estilo das lives existentes.
2. Criar ou substituir `live-N.html` seguindo o padrao visual do projeto.
3. Criar `live-N.ipynb` com exemplos executaveis e narrativa didatica.
4. Atualizar `main.html` da disciplina com o tema correto da aula.
5. Validar o notebook executando as celulas fora do Jupyter quando possivel.

