# ANTENA — primeras diez tareas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · **Español** · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créalas en GitHub con `scripts/create-issues.sh OWNER/REPO`. Comenta en una issue para reclamarla.

1. **Analiza las tarjetas restantes: CM, CE, GE, EX, FR, RP, EN** — `good first issue`, `area:parser`
   Una función por tarjeta, en el mismo estilo que `parse_gw`; pruebas para tarjetas correctas y mal formadas; un tipo `Deck` que contenga todas las tarjetas en orden.

2. **Validación de la geometría: longitud de segmento, radio, intersecciones de hilos** — `good first issue`, `area:geometry`
   Avisa cuando un segmento supere λ/10 o el radio > segmento/8 (cita la fuente de la regla). Devuelve avisos, nunca entres en pánico.

3. **Relleno de la matriz de impedancia en espacio libre (base de pulsos, núcleo de hilo fino)** — `help wanted`, `area:solver`
   Documenta la formulación con referencias a las ecuaciones; prueba la simetría y un caso de dos segmentos calculado a mano.

4. **Resolución lineal (LU propia con pivoteo parcial o un crate permisivo)** — `help wanted`, `area:solver`
   Matrices complejas; protección del número de condición; prueba con un sistema complejo 3×3 conocido.

5. **Impedancia de entrada del dipolo de media onda: valida con valores publicados** — `help wanted`, `tests`, `area:solver`
   Dipolo delgado, 21 segmentos: parte real ≈ 73 Ω ± tolerancia declarada y justificada; cita la referencia.

6. **Diagrama de radiación (tarjeta RP): campo lejano, ganancia en dBi** — `help wanted`, `area:solver`
   Barrido en azimut/elevación; compara la forma del diagrama del dipolo con la analítica.

7. **Suelo perfecto con el método de las imágenes** — `help wanted`, `area:ground`
   Monopolo vertical sobre suelo perfecto ≈ 36 Ω; el suelo Sommerfeld/Norton, en una issue posterior.

8. **Salida: informe de texto al estilo NEC-2 y JSON** — `good first issue`, `area:output`
   Esquema JSON estable en `docs/`; pruebas con archivos de referencia.

9. **Compilación WebAssembly y visor en canvas para geometría y diagramas 2D** — `help wanted`, `area:wasm`
   Página estática en `docs/demo/`; pega un deck y ve los hilos y el diagrama.

10. **Contacta con el Open Research Institute sobre Arcanum: modelos de prueba compartidos y licencias** — `help wanted`, `docs`
   Abre una discusión en el proyecto original, resume el resultado en `docs/COORDINATION.md` (qué se comparte, qué queda separado).
