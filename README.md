# ♟️ Chess-TUI 

**Chess-TUI** es una aplicación de ajedrez moderna para terminal, desarrollada en Rust, que utiliza **Ratatui** y **Crossterm** para la interfaz de usuario.
El proyecto sigue una arquitectura modular, extensible y con separación estricta de responsabilidades.

---

## 🧱 Arquitectura general

El código se organiza en los siguientes módulos:

- **`main.rs`** → Punto de entrada, bucle principal de eventos, alternancia de pantalla.
- **`app.rs`** → Orquestador: estado global, máquina de estados de la UI, manejo de entradas (ratón, teclado, tick).
- **`config.rs`** → Carga y almacenamiento de la configuración (JSON).
- **`events.rs`** → Definición de eventos unificados (teclado, ratón, tick, redimensionado, red).
- **`states.rs`** → Máquina de estados de la interfaz (`Idle`, `PieceSelected`, etc.).
- **`engine/`** → Motor de ajedrez independiente de la UI.
  - `board.rs` → Representación del tablero (array 8×8), gestión de casillas, aplicación de movimientos.
  - `moves.rs` → Estructura de movimiento, notación algebraica básica.
  - `game.rs` → Lógica de juego: generación de movimientos pseudo-legales y legales, detección de jaque/jaque mate/ahogado, enroque, captura al paso, promoción.
  - `fen.rs` → Serialización/deserialización FEN (carga de posición inicial).
- **`ai/`** → Módulo de inteligencia artificial.
  - `mod.rs` → Trait `ChessAi` e implementación para nivel 1 (movimiento aleatorio).
- **`ui/`** → Capa de presentación con Ratatui.
  - `layout.rs` → Cálculo de los tres paneles (izquierdo, tablero central cuadrado, derecho).
  - `board_renderer.rs` → Renderizado del tablero con piezas Unicode, colores del tema, resaltado de selección y movimientos legales.
  - `theme.rs` → Definición del tema **Tokio Night** (colores suaves y modernos).
  - `mouse.rs` → Conversión de coordenadas del ratón a casillas del tablero.
  - `widgets.rs` → Paneles laterales: historial de partidas, información de jugadores, relojes, lista de movimientos.
- **`network/`** → Módulo placeholder para red P2P (pendiente de implementar).
- **`storage/`** → Persistencia del historial de partidas en archivos JSON.
- **`utils/`** → Utilidades: reloj de ajedrez (`time.rs`).

---

## 🎮 Interfaz de usuario

### Tema visual
- **Tokio Night**: colores suaves y oscuros, con casillas claras (#2e3440) y oscuras (#3b4252).
- Piezas blancas en color claro, piezas negras en color oscuro.
- Resaltados:
  - Selección de pieza: azul (#5e81ac).
  - Movimientos posibles: verde (#a3be8c).
  - (Captura y último movimiento coloreados pero no completamente integrados aún).

### Distribución de pantalla
- **Panel izquierdo (25%)**: historial de partidas (fecha, rival, resultado). Navegable con ratón (implementación parcial).
- **Panel central (50%)**: tablero ajustable al tamaño del terminal. Cada casilla ocupa múltiples caracteres (ancho de celda variable). Piezas en Unicode (♔♕♖♗♘♙ / ♚♛♜♝♞♟).
- **Panel derecho (25%)**: nombre de jugadores, relojes (mm:ss), lista de movimientos en notación algebraica (e.g., e2e4).

### Interacción (Por terminar de desarrollar)
- **Control exclusivo con ratón** (clics izquierdos).
- Flujo de selección:
  - Clic en una pieza propia → se selecciona y se muestran sus movimientos legales.
  - Clic en una casilla válida → se ejecuta el movimiento.
  - Clic en la misma pieza o fuera → se cancela la selección.
  - Clic en pieza ilegal → no ocurre nada (sin mensajes de error).
- Tecla `Esc` para salir.
- **Máquina de estados** definida en `states.rs`: `Idle`, `PieceSelected`, `WaitingOpponent`, `GameFinished` (y otras previstas).

---

## ⚙️ Motor de ajedrez

Implementado completamente en `engine/` y sin dependencias de la UI.

### Funcionalidades incluidas
- **Generación de movimientos**:
  - Peón: avance simple, doble desde la fila inicial, capturas diagonales, promoción (dama, torre, alfil, caballo), captura al paso.
  - Caballo: movimientos en L.
  - Alfil, torre, dama: deslizantes con bloqueo.
  - Rey: movimientos unitarios.
- **Reglas especiales**:
  - **Enroque** (corto y largo) para ambos bandos, con todas las condiciones (casillas libres, no estar en jaque ni pasar por jaque, rey y torre no movidos previamente).
  - **Captura al paso** correctamente implementada.
  - **Promoción** del peón al llegar a la octava fila (se permite elegir entre las cuatro piezas).
- **Validación**:
  - Filtrado de movimientos pseudo-legales para obtener solo los **legales** (no dejan al propio rey en jaque).
- **Detección de final de partida**:
  - **Jaque**: se detecta cuando el rey está atacado.
  - **Jaque mate**: rey en jaque sin movimientos legales.
  - **Ahogado**: sin movimientos legales y sin jaque (tablas).
  - (Otros tipos de tablas como triple repetición o regla de 50 movimientos no están implementados todavía).
- **FEN**: carga de la posición inicial desde FEN, y posibilidad de serializar (método `to_fen` incompleto pero presente).

---

## 🧠 Inteligencia artificial (por desarrollar correctamente)

- **Nivel 1 implementado**: movimiento aleatorio entre todos los legales.
- Arquitectura preparada para niveles adicionales mediante el trait `ChessAi`.
- Futuros niveles (evaluación material, minimax, alpha-beta, tablas de transposición) se podrán añadir sin modificar el resto del código.

---

## ⏱️ Reloj de ajedrez

- **Control de tiempo**: cada jugador comienza con 10 minutos (configurable en `config.time_minutes`).
- Cambio de turno automático al realizar un movimiento.
- Cuenta regresiva en tiempo real (actualización cada 100 ms).
- Se muestra en el panel derecho en formato `mm:ss`.
- Si un jugador se queda sin tiempo, se declara pérdida por tiempo (aún no se guarda en el historial automáticamente).

---

## 💾 Persistencia

### Configuración
- Archivo `config.json` en directorio de configuración del sistema (`dirs::config_dir/chess-tui/`).
- Valores guardados:
  - `player_name`
  - `time_minutes`
  - `ai_level`
  - `port`
  - `theme`
- Se carga al iniciar y se guarda al salir.

### Historial de partidas
- Archivo `history.json` en directorio de datos (`dirs::data_dir/chess-tui/`).
- Cada entrada contiene:
  - Fecha y hora (`DateTime<Utc>`)
  - Nombres de jugadores (blancas y negras)
  - Resultado (ej. "White wins by checkmate", "Stalemate - Draw")
  - Lista de movimientos (notación algebraica simple)
- Las partidas se guardan **automáticamente al finalizar** (no implementado aún el guardado automático, pero la estructura existe y se puede llamar manualmente).
- El panel izquierdo muestra el historial cargado al inicio.

---

## 🌐 Red (Por hacer)

- Módulo `network/` presente pero vacío.
- Estructura preparada para implementar comunicación TCP peer-to-peer y descubrimiento UDP.
- La arquitectura permite añadir partidas en red sin modificar la UI ni el motor.

---

## 🧪 Cobertura de reglas de ajedrez

| Regla                | Implementada |
|----------------------|--------------|
| Movimientos básicos  | ✅ Sí        |
| Captura al paso      | ✅ Sí        |
| Enroque (corto/largo)| ✅ Sí        |
| Promoción (4 piezas) | ✅ Sí        |
| Jaque                | ✅ Sí        |
| Jaque mate           | ✅ Sí        |
| Ahogado              | ✅ Sí        |
| Triple repetición    | ❌ No        |
| Regla 50 movimientos | ❌ No        |
| FEN (carga)          | ✅ Sí        |
| PGN (exportación)    | ❌ No        |

---

## 🧹 Buenas prácticas de Rust

- Separación estricta entre motor, UI, almacenamiento e IA.
- Uso extensivo del sistema de ownership y préstamos.
- Minimización de clones innecesarios (se usan referencias siempre que es posible).
- Manejo robusto de errores con `Result` y `anyhow`.
- Código documentado con comentarios.
- Evitación de `unwrap` y `panic` en la lógica principal.
- Máquina de estados con enums en lugar de booleanos.

---

## 🚀 Cómo ejecutar

```bash
cargo run

La aplicación se inicia en modo raw, con pantalla alternada y soporte de ratón. Al salir (Esc) se restaura la terminal.
🔮 Próximos pasos

    Implementar niveles de IA superiores (minimax, alpha-beta, transposición).

    Añadir partidas en red (TCP + UDP discovery).

    Mejorar el guardado automático de partidas al finalizar.

    Soporte para reproducción de partidas históricas.

    Interfaz más completa para menús (nueva partida IA, online, unirse a sala).

    Exportación PGN y carga de posiciones personalizadas.

    Personalización de colores y más temas.

Versión actual: 0.1.0
Licencia: MIT / Apache-2.0
Repositorio: [https://github.com/Adri-Coding-Dev/Chess-TUI]
