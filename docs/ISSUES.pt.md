# ANTENA — primeiras dez tarefas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · **Português** · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Crie-as no GitHub com `scripts/create-issues.sh OWNER/REPO`. Comente numa issue para a reivindicar.

1. **Analise os cartões restantes: CM, CE, GE, EX, FR, RP, EN** — `good first issue`, `area:parser`
   Uma função por cartão, no mesmo estilo de `parse_gw`; testes para cartões corretos e malformados; um tipo `Deck` que guarde todos os cartões em ordem.

2. **Validação da geometria: comprimento do segmento, raio, interseções de fios** — `good first issue`, `area:geometry`
   Avise quando um segmento exceder λ/10 ou o raio > segmento/8 (cite a fonte da regra). Devolva avisos, nunca entre em pânico.

3. **Preenchimento da matriz de impedância no espaço livre (base de pulsos, núcleo de fio fino)** — `help wanted`, `area:solver`
   Documente a formulação com referências às equações; teste a simetria e um caso de dois segmentos calculado à mão.

4. **Resolução linear (LU próprio com pivotamento parcial ou um crate permissivo)** — `help wanted`, `area:solver`
   Matrizes complexas; proteção do número de condição; teste com um sistema complexo 3×3 conhecido.

5. **Impedância de entrada do dipolo de meia onda: valide com valores publicados** — `help wanted`, `tests`, `area:solver`
   Dipolo fino, 21 segmentos: parte real ≈ 73 Ω ± tolerância declarada e justificada; cite a referência.

6. **Diagrama de radiação (cartão RP): campo distante, ganho em dBi** — `help wanted`, `area:solver`
   Varredura em azimute/elevação; compare a forma do diagrama do dipolo com a analítica.

7. **Solo perfeito pelo método das imagens** — `help wanted`, `area:ground`
   Monopolo vertical sobre solo perfeito ≈ 36 Ω; o solo Sommerfeld/Norton numa issue posterior.

8. **Saída: relatório de texto ao estilo NEC-2 e JSON** — `good first issue`, `area:output`
   Esquema JSON estável em `docs/`; testes com arquivos de referência.

9. **Build WebAssembly e visualizador em canvas para geometria e diagramas 2D** — `help wanted`, `area:wasm`
   Página estática em `docs/demo/`; cole um deck e veja os fios e o diagrama.

10. **Contate o Open Research Institute sobre o Arcanum: modelos de teste compartilhados e licenças** — `help wanted`, `docs`
   Abra uma discussão a montante, resuma o resultado em `docs/COORDINATION.md` (o que é compartilhado, o que fica separado).
