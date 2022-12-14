# Heliochrome
Heliochrome is a CPU ray tracer written in rust. This was initially adapted from Peter Shirley's [Ray Tracing in One Weekend Series](https://raytracing.github.io/), and has been extended in my exploration of ray tracing.

## References
* [Ray Tracing in One Weekend Series](https://raytracing.github.io/) (Peter Shirley)
* [Inigo Quilez's whole site](https://iquilezles.org/) (Inigo Quilez)
* [https://64.github.io/tonemapping/](https://64.github.io/tonemapping/) (Matt Taylor)

## Features
* Semi-real time previewing of renders using a cumulative image buffer
* Mesh primitive and loading from obj files
* Signed Distance Function (SDF) primitives using ray marching
* Homogeneous transforms using matrices and a custom maths module
* HDRI equirectangular skyboxes
* Tone mapping using Hable, Reinhard, Hejl-Richard, and ACES
* Custom scene loader

## Results
<figure>
    <p  align="center">
        <img src="results/smoothanne.png" />
        <figcaption><p align="center">smoothanne, 1000 samples (with importance sampling)</p><figcaption/>
    </p>
</figure>
<br/>
<figure>
    <p  align="center">
        <img src="results/cubes.png" />
        <figcaption><p align="center">cubes, 200 samples</p><figcaption/>
    </p>
</figure>
<br/>
<figure>
    <p  align="center">
        <img src="results/swirlyboi.png" />
        <figcaption><p align="center">swirlyboi, 300 samples</p><figcaption/>
    </p>
</figure>
<br/>
<figure>
    <p  align="center">
        <img src="results/mandel_bulb.png" />
        <figcaption><p align="center">mandel bulb, 250 samples (Reinhard tone mapping)</p><figcaption/>
    </p>
</figure>
<br/>
<figure>
    <p  align="center">
        <img src="results/glass_suzanne.png" />
        <figcaption><p align="center">glass suzanne, 500 samples</p><figcaption/>
    </p>
</figure>
<br/>
<figure>
    <p  align="center">
        <img src="results/orbs.png" />
        <figcaption><p align="center">orbs, 1200 samples</p><figcaption/>
    </p>
</figure>
<br/>
<figure>
    <p  align="center">
        <img src="results/suzanne.png" />
        <figcaption><p align="center">suzanne (no smooth shade), 400 samples</p><figcaption/>
    </p>
</figure>
<br/>
<figure>
    <p  align="center">
        <img src="results/box.png" />
        <figcaption><p align="center">cornell box, 1000 samples (with importance sampling)</p><figcaption/>
    </p>
</figure>
<br/>