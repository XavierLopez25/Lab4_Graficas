# Sistema Solar en Rust

Este repositorio contiene un proyecto de simulación del sistema solar inspirado en el juego Warframe, utilizando Rust para renderizar gráficamente los cuerpos celestes, incluyendo efectos de shaders personalizados y modelos de anillos para los planetas gaseosos.

## Descripción del Proyecto

El sistema solar se representa mediante un renderer de software en Rust, donde cada planeta y el sol se muestran con efectos visuales únicos, simulados a través de shaders sin emplear texturas externas. La representación incluye detalles como atmósferas, superficies rocosas o gaseosas, anillos planetarios y otros fenómenos visuales.

## Sistema Solar de Warframe
![image](https://github.com/user-attachments/assets/8aa45cb6-602d-4ea6-9bba-8cc127054ba8)


## Videos de los planetas de Warframe en el simulador de Rust

### Sol (no visible en Warframe, pero presente)
https://github.com/user-attachments/assets/113da9b7-7990-4178-b1e1-6f49560d9cfb
---
#### Tierra y Lua
https://github.com/user-attachments/assets/1f7691e3-7bd2-4bbe-87f0-7d00b3ef4edc
---
### Venus




### Características

- Representación del Sol y varios planetas, cada uno con su propio conjunto de shaders y efectos visuales.
- Uso de modelos para los anillos de los planetas gaseosos, sin shaders para los anillos.
- Implementación de un sistema de movimiento y rotación para simular la órbita y rotación de los cuerpos celestes.
- Control de cámara interactivo para explorar el sistema solar.

### Planetas Incluidos

- **Sol**: Efectos de brillo y llamaradas simuladas con shaders.
- **Planetas rocosos y gaseosos**: Desde Mercurio hasta Sedna, cada uno con características únicas.
- **Anillos de Saturno y Urano**: Modelados con objetos específicos y no mediante shaders.

## Instalación y Uso

### Prerrequisitos

Asegúrate de tener Rust y Cargo instalados en tu sistema. Puedes instalarlos desde [la página oficial de Rust](https://www.rust-lang.org/tools/install).

### Ejecutar el Proyecto

1. Clona este repositorio:
   ```bash
   git clone [https://github.com/tu_usuario/sistema-solar-rust.git](https://github.com/XavierLopez25/Lab4_Graficas)
   cd Lab4_Graficas
   ```

2. Compila y ejecuta el proyecto:
   ```bash
   cargo build --release
   cargo run --release
   ```

### Controles

- **Movimiento de cámara**: Flechas para rotar la vista.
- **Zoom**: Teclas `W` y `S` para acercar y alejar.
- **Salir**: `Esc` para cerrar la aplicación.

## Detalles Técnicos

- **Renderer**: Utiliza `minifb` para la ventana y el dibujo pixel por pixel.
- **Shaders**: Cada cuerpo celeste utiliza shaders escritos en Rust para definir su apariencia.
- **Modelos 3D**: Carga modelos de esferas y anillos usando `tobj`.

## Librerías Usadas

- `fastnoise-lite`: Para generar ruido en los shaders.
- `minifb`: Para la creación de ventanas y manejo de eventos.
- `nalgebra-glm`: Para cálculos matemáticos de gráficos.
- `rand`: Utilizado en la generación de algunas características aleatorias.
- `tobj`: Para cargar modelos 3D.


Puedes ver videos de cada planeta en acción en la sección de [Videos de Planetas](#) en este repositorio.
```

Este `README` proporciona una visión general del proyecto, instrucciones de instalación y uso, controles para interactuar con la simulación, detalles técnicos sobre la implementación y las librerías usadas, así como una sección dedicada a videos de los planetas, que deberás actualizar con enlaces reales a los videos si los agregas posteriormente.
