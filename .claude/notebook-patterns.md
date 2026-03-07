# Padrao de Notebooks

## Localização e Nome

- Cada notebook fica ao lado da live correspondente:
  - `pages/<disciplina>/live-N.ipynb`

## Estrutura Recomendada

1. Titulo da aula
2. Subtitulo com disciplina
3. Introducao com objetivos
4. Setup inicial
5. Blocos alternados de explicacao e codigo
6. Exemplo principal da aula
7. Aplicacoes de vida real
8. Conclusao

## Setup Inicial

Preferir o minimo necessario:

```python
import numpy as np
import matplotlib.pyplot as plt

plt.style.use('seaborn-v0_8-darkgrid')
np.set_printoptions(precision=4, suppress=True)
```

Se a aula exigir outra biblioteca, manter o setup enxuto e validar a disponibilidade.

## Estilo Didático

- Misturar markdown e codigo.
- Explicar primeiro a intuicao, depois a implementacao.
- Usar exemplos concretos e nao so abstratos.
- Mostrar saidas e visualizacoes quando ajudarem na compreensao.

## Conteúdo Esperado

- O notebook nao deve ser so "codigo solto".
- Cada secao precisa responder uma pergunta:
  - qual problema estamos modelando?
  - por que usar essa estrutura?
  - como a implementacao funciona?
  - onde isso aparece na vida real?

## Boas Práticas Técnicas

- Evitar dependencias desnecessarias.
- Se houver aleatoriedade, usar `seed`.
- Se possivel, validar o notebook executando todas as celulas fora do Jupyter.
- Manter JSON do notebook valido.

## Casos Reais

Sempre incluir pelo menos um caso aplicado, por exemplo:

- `applied-math`: treinamento de modelo, previsao temporal, simulacao
- `data-structure`: ranking, catalogo, indice, fila de processamento, sistema de busca

