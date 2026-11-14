# ANTENA — Solucionador de antenas de fio compatível com NEC-2 (método dos momentos), com interface no navegador.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · **Português** · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versão: 0.1.0-alpha.1 · Última atualização: 2026-11-14

🌐 https://antena.chiuta.ro

## Estado

Esqueleto inicial (pré-alfa): o workspace compila e os testes passam no CI. Cada funcionalidade prevista é uma tarefa aberta em `docs/issues/`.

## Âmbito previsto

- Leitura dos cartões NEC-2 em falta (CM, CE, GE, EX, FR, RP, EN) e validação da geometria.
- Método dos momentos para fios finos: preenchimento da matriz de impedâncias e resolvedor linear.
- Impedância de entrada do dipolo de meia onda verificada com valores publicados.
- Diagrama de radiação (cartão RP), ganho em dBi e solo perfeito pelo método das imagens.
- Relatório de texto ao estilo NEC-2 mais JSON, e compilação WebAssembly com visualizador canvas para a geometria e os diagramas 2D.

Nada disto está ainda implementado: cada ponto é uma tarefa aberta em `docs/issues/`.

## Compilar e testar

```bash
cargo test --workspace
cargo run -p antena-cli -- dipole 14.2
```

## Contribuir

Leia `CONTRIBUTING.md`, escolha uma tarefa com a etiqueta `good first issue` em `docs/issues/` e abra um pull request. Nenhuma contribuição é demasiado pequena: uma linha de tabela, um teste ou uma tradução contam.

## Implementação independente

Escrita a partir de especificações públicas e manuais; não foi copiado código-fonte de terceiros (ver `CLEANROOM.md`).

## Licença

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoio

Trade-free: sem rastreamento nem publicidade. Se lhe for útil, pode apoiar o autor: https://buymeacoffee.com/echo.of.the.strings
