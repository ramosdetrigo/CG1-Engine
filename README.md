# CG1-Engine
Trabalho final da disciplina de Computação Gráfica 1\
Linguagem utilizada: Rust\
Bibliotecas utilizadas:
- [SDL2](https://crates.io/crates/sdl2) para renderizar coisas numa janela
- [imgui](https://crates.io/crates/imgui) para interface de usuário
  - [imgui-sdl2-support](https://crates.io/crates/imgui-sdl2-support) suporte do imgui para SDL
  - [imgui-glow-renderer](https://github.com/imgui-rs/imgui-glow-renderer) renderer usado pelo imgui-sdl2-support
- [obj-rs](https://crates.io/crates/obj-rs) para carregar arquivos .obj


## Como rodar:
Requer a ferramenta `cargo` e a biblioteca `sdl2` instaladas no sistema.\
Para instalar sdl2 no linux, basta instalar o pacote `libsdl2-dev` via apt (ou o equivalente para distros não-debian)\
Este projeto também requer a biblioteca SDL2_image instalada (`libsdl2-image-dev`)\
Para instalar rustup, cargo, e as ferramentas de rust, basta seguir o [guia do site oficial](https://www.rust-lang.org/tools/install)\
Para compilar e rodar, use o seguinte comando na raiz do projeto:
```
cargo run --release
```
