1. [x] **Definição do cenário**
    1. [x] Coerência temática (Obrigatório)
	    `O cenário tem de ter coerência temática, isto é, não pode ser um amontoado de objetos aleatoriamente distribuídos.`
	2. [x] Coordenadas do mundo (Obrigatório)
		`O cenário deve ser montado de forma que todos os objetos estejam no primeiro octante, isto é, as coordenadas dos vértices de todos os objetos terão x, y e z positivos.`
	3. [x] Objetos
		1. [x] Esfera
		2. [x] Cilindro
		3. [x] Cone
		4. [x] Malha
	4. [x] Materiais (pelo menos quatro materiais distintos) (Obrigatório)
	5. [x] Textura (pelo menos uma textura aplicada) (Obrigatório)
	6. [x] Transformações
		1. [x] Translação (Obrigatório)
		2. [x] Rotação (Obrigatório)
			1. [x] Em torno de um dos eixos x, y ou z (Obrigatório)
			2. [x] Em torno de um eixo arbitrário (Quatérnios/mudança de sist. de coord.)
		3. [x] Escala (Obrigatório)
		4. [x] Cisalhamento (+ 0.5)
		5. [x] Espelho em relação a um plano arbitrário (+ 0.5)
	7. [x] Fontes luminosas
		1. [x] Pontual (Obrigatório)
		2. [x] Spot (+1.0)
		3. [x] Direcional (+0.5)
		4. [x] Ambiente (Obrigatório)
2. [x] **Câmera**
	1. [x] Posição da câmera (Eye)
	2. [x] Direcionamento de visada (At point)
	3. [x] Orientação da câmara em torno do eixo de visada (Up point)
	4. [x] Distância focal (d)
	5. [x] Campo de visão (definir as coordenadas de câmera da janela: xmin, xmax, ymin, ymax)   
3. [x] **Projeções**
	1. [x] Perspectiva (Obrigatório)  
		1. [x] aumentar o campo de visão (zoom out) (Obrigatório)
		2. [x] diminuir o campo de visão (zoom in) (Obrigatório)
		3. [x] Demonstrar como posicionar a câmerapara obter
			1. [x] Perspectiva com um ponto de fuga (+0.5)
			2. [x] Perspectiva com dois pontos de fuga (+ 0.5)
			3. [x] Perspectiva com três ou mais pontos de fuga (+ 0.5)
	2. [x] Ortográfica (+ 0.5)
	3. [x] Obliqua (+0.5)
4. [x] **Sombra (Obrigatório)**
5. [x] **Interatividade**
	1. [x] Implementar a função de pick (Obrigatório)
	2. [x] Uso de interface gráfica (Bônus de 0.5 a 1.0)
6. [x] **Imagem gerada por ray casting com pelo menos 500 x 500 pixels (Obrigatório)**
7. [x] **Bônus de criatividade e beleza (até 1.0)**
