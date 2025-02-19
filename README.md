# CG1-Engine
Atividades feitas durante a disciplina de Computação Gráfica 1\
Linguagem utilizada: Rust\
Bibliotecas utilizadas:
- [SDL2](https://crates.io/crates/sdl2) para renderizar coisas numa janela
- [obj-rs](https://crates.io/crates/obj-rs) para carregar arquivos .obj


## Como rodar:
Requer a ferramenta `cargo` e a biblioteca `sdl2` instaladas no sistema.\
Para instalar sdl2 no linux, basta instalar o pacote `libsdl2-dev` via apt (ou o equivalente para distros não-debian)\
Para instalar rustup, cargo, e as ferramentas de rust, basta seguir o [guia do site oficial](https://www.rust-lang.org/tools/install)\
Para compilar e rodar, use o seguinte comando na raiz do projeto:
```
cargo run
```
Para compilar e rodar como release (bin otimizado), use:
```
cargo run --release
```
