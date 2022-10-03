# Heliochrome
Heliochrome is a CPU ray tracer written in rust. This was initially adapted from Peter Shirley's [Ray Tracing in One Weekend Series](https://raytracing.github.io/), and has been extended in my exploration of ray tracing.

## References
* [Ray Tracing in One Weekend Series](https://raytracing.github.io/) (Peter Shirley)
* [Inigo Quilez's whole site](https://iquilezles.org/) (Inigo Quilez)
* [https://64.github.io/tonemapping/](https://64.github.io/tonemapping/) (Matt Taylor)

## Features
* Semi-real time previewing of renders using a cumulative image buffer
* Mesh primitive and loading from obj files
* SDF primitive
* linear transforms using matrices and a custom maths module
* hdri equirectangular skyboxes
* Tone mapping using Hable, Reinhard, Hejl-Richard, and ACES

## Results
<p align="center">
    <img src="results/cubes.png">
    <br/>cubes, 200 samples<br/>
    <img src="results/swirlyboi.png">
    <br/>swirlyboi, 300 samples<br/>
    <img src="results/glass_suzanne.png">
    <br/>glass suzanne, 500 samples<br/>
    <img src="results/orbs.png">
    <br/>orbs, 1200 samples<br/>
    <img src="results/suzanne.png">
    <br/>suzanne, 400 samples<br/>
</p>