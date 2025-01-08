# path-tracer

Implementation of a path tracer in Rust

![dof](img/dof.png)

![glass](img/glass.png)

## Features

- Render pipeline:
  - Shapes
    - Sphere
    - Triangle
    - World (collection of shapes)
  - Materials
    - Lighting
    - Transparency
    - Metals
    - Glass
    - Lambertians
    - Dielectrics
  - Camera
    - FOV
    - Focal length
    - Position
- Image:
  - In-memory buffer of canvas data
  - Utility methods to iterate over each `(x, y)` pixel
  - Buffered write of pixel data, reaching ≈11k pixels-per-millisecond (p/ms) on M1 Max
- Scene
  - Save scene to file
  - Load scene from file
  - Scene data
    - Render settings
    - Image resolution
    - Camera position
    - Object placement
