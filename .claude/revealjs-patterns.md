# Padrao Reveal.js

## Estrutura Base

- Cada live fica em `pages/<disciplina>/live-N.html`.
- Usar `<!doctype html>` e `lang="pt-BR"`.
- Incluir:
  - Bootstrap 5.3.3
  - `reveal.css`
  - tema de `Reveal.js`, normalmente `black.css`
- Estrutura minima:

```html
<body>
  <div class="reveal">
    <div class="slides">
      <section>...</section>
    </div>
  </div>

  <script src="https://cdn.jsdelivr.net/npm/reveal.js@4.5.0/dist/reveal.js"></script>
  <script>
    Reveal.initialize({
      hash: true,
      slideNumber: true,
      transition: 'slide'
    });
  </script>
</body>
```

## Convenções Visuais

- Tema escuro por padrao.
- Titulos sem `text-transform`.
- Cards com fundo em gradiente escuro, borda leve e sombra.
- Cores de destaque recorrentes:
  - ciano para foco estrutural
  - verde para estado positivo
  - amarelo para comparacoes
  - rosa/vermelho para alerta ou contraste

## Organização da Aula

Uma live de 1h30 normalmente precisa de um volume maior de slides. Estrutura recomendada:

1. Capa
2. Motivacao
3. Conceito central
4. Visual ou analogia
5. Operacoes principais
6. Exemplo de codigo
7. Situacoes da vida real
8. Comparacoes e trade-offs
9. Caso pratico
10. Resumo

## Boas Práticas

- Preferir 1 ideia principal por slide.
- Evitar paredes de texto.
- Quando houver 3 colunas, reduzir texto e formulas para nao quebrar o layout.
- Para matematica simples, preferir texto estilizado se `MathJax` comprometer o layout.
- Para explicacoes longas, usar `grid-2` em vez de muitas colunas estreitas.
- Se houver risco de quebra visual, renderizar localmente com navegador headless antes de finalizar.

## Elementos Úteis

- `.hero-box` para capa
- `.glass-card` para cards conceituais
- `.equation-box` para regras curtas
- `.notes-row` com pequenos blocos de observacao
- SVG inline para ilustrar arvores, grafos, curvas e comparacoes

## Conteúdo

- Usar portugues claro e didatico.
- Incluir exemplos do mundo real em quase toda aula.
- Explicar trade-offs, nao so definicoes.
- Sempre ligar estrutura de dados ao tipo de problema que ela resolve.

