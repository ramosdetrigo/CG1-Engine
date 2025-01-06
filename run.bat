@echo off
g++ -O3 -march=native -IE:\SDL2\SDL2-2.30.9\include -D_REENTRANT -o cg1.exe main.cpp -LE:\SDL2\SDL2-2.30.9\lib\x64 -lSDL2
cg1.exe
