# pathracer-wgpu

Toy project pathracer with wgpu, winit, egui

## Status

Raytracer one weekend has been fully implemented, goal will be to implement everything from [raytracer book](https://raytracing.github.io/)

![prev](image/raytracer_oneweekend.png)

## Build to WASM

```
cargo install trunk
trunk build --features webgpu
trunk serve --features webgpu
```

<!-- Will not work currently
```
wasm-pack build --target web pathracer-wgpu
``` -->

## Run to web

### NextJS project

TODO
