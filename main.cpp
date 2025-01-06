#include <iostream>
#include <cmath>
#include <chrono>
#include <SDL.h>

#include "utils/vec3.h"
#include "engine/camera.h"
#include "engine/light.h"
#include "engine/scene.h"
#include "engine/shapes/material.h"
#include "engine/shapes/sphere.h"
#include "engine/shapes/plane.h"


using namespace std;

int main() {
    Vec3 p0 = Vec3(0,0,0);
    
    double aspect_ratio = 16.0/9.0;
    double viewport_width = 3.2;
    double viewport_height = viewport_width/aspect_ratio;
    double viewport_distance = 1.0;
    int image_width = 960;
    int image_height = image_width/aspect_ratio;

    double sphere_radius = 1.0;
    Vec3 sphere_center = Vec3(0,0, -(viewport_distance + sphere_radius));

    Vec3 plane_p0 = Vec3(0.0, -1.8, 0.0);
    Vec3 plane_normal = Vec3(0.0, 1.0, 0.0);

    Vec3 plane2_p0 = Vec3(0.0, 0.0, -6.0);
    Vec3 plane2_normal = Vec3(0.0, 0.0, 1.0);
    
    Vec3 bg_color = Vec3(0.0, 0.0, 0.0);
    Material mat_sphere = Material(
        Vec3(0.7, 0.2, 0.2),
        Vec3(0.7, 0.2, 0.2),
        Vec3(0.7, 0.2, 0.2),
        10
    );
    Material mat_p1 = Material(
        Vec3(0.2, 0.7, 0.2),
        Vec3(0.2, 0.7, 0.2),
        Vec3(0.0, 0.0, 0.0),
        1
    );
    Material mat_p2 = Material(
        Vec3(0.3, 0.3, 0.7),
        Vec3(0.3, 0.3, 0.7),
        Vec3(0.0, 0.0, 0.0),
        1
    );

    Sphere* sphere = new Sphere(sphere_center, sphere_radius, mat_sphere);
    Plane* plane = new Plane(plane_p0, plane_normal, mat_p1);
    Plane* plane2 = new Plane(plane2_p0, plane2_normal, mat_p2);

    Light light1 = Light(
        Vec3(-0.8, 0.8, 0.0),
        Vec3(1.0, 0.0, 0.0),
        0.7
    );
    Light light2 = Light(
        Vec3(0.8, 0.8, 0.0),
        Vec3(0.0, 0.0, 1.0),
        0.7
    );

    Vec3 ambient_light = Vec3(0.3, 0.3, 0.3);

    Camera camera = Camera(p0, viewport_width, viewport_height, image_width, image_height, viewport_distance, bg_color);

    Scene scene = Scene(ambient_light);
    scene.add_object(sphere);
    scene.add_object(plane);
    scene.add_object(plane2);
    scene.add_light(light1);
    scene.add_light(light2);

    // SDL init
    if (SDL_Init(SDL_INIT_VIDEO) != 0) { printf("SDL_Init Error: %s\n", SDL_GetError()); return 1; }
    SDL_Window* window = SDL_CreateWindow("Hello SDL", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, image_width, image_height, 0);
    if (window == NULL) { printf("SDL_CreateWindow Error: %s\n", SDL_GetError()); SDL_Quit(); return 1; }
    SDL_Renderer* renderer = SDL_CreateRenderer(window, -1, SDL_RENDERER_ACCELERATED);
    if (renderer == NULL) { printf("SDL_CreateRenderer Error: %s\n", SDL_GetError()); SDL_DestroyWindow(window); SDL_Quit(); return 1; }

    // contador de fps
    int frameCount = 0;
    auto startTime = std::chrono::high_resolution_clock::now();
    // main loop
    SDL_Event event;
    while (true) {
        // event handler
        while (SDL_PollEvent(&event) != 0) {
            if (event.type == SDL_QUIT || (event.type == SDL_KEYDOWN && event.key.keysym.sym == SDLK_ESCAPE)) {
                goto quit;
            }
        }

        // draw sphere
        camera.draw_scene(renderer, scene);

        // printa o FPS no terminal
        frameCount++;
        auto currentTime = std::chrono::high_resolution_clock::now();
        std::chrono::duration<double> elapsedTime = currentTime - startTime;
        if (elapsedTime.count() >= 1.0) {
            std::cout << "FPS: " << frameCount << std::endl;
            frameCount = 0;
            startTime = currentTime;
        }
    }
    quit:

    delete sphere;
    // SDL_DestroyRenderer(renderer);
    SDL_DestroyWindow(window);
    SDL_Quit();

    return 0;
}
        