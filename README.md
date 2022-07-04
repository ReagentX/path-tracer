# path-tracer
Implementation of a path tracer in Rust

![](img/dof.png)

![](img/glass.png)

# Features

- Render pipeline:
    - Shapes
        - Sphere
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
    - Buffered write of pixel data, reaching ≈9.5k pixels-per-milisecond (p/ms) on i7-6700K
- Scene
    - Save scene to file
    - Load scene from file
    - Scene data
        - Render settings
        - Image resolution
        - Camera position
        - Object placement
