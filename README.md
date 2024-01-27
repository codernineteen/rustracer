# Pure Rust implementation for ray-tracing-series

## Implementation list
1. Basic ray tracer
- module structure
```
├── lib.rs
└── components (useful abstractions for components in a scene. For example, camera)
    ├── camera.rs 
└── geometry (geometry primitives)
    ├── sphere.rs
    └── cube.rs
├── linalg (linear algebra module)
    └── vector.rs
├── material (material abstractions)
└── perf
```

- features
    - diffuse reflection 
    - specular reflection
    - refraction
    - multi-threaded rendering

## References
- All implementation details based on this awesome series -> [raytracing.github.io](https://raytracing.github.io/)