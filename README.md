# 🦀 Ray Tracing in One Weekend (Rust Edition)

This repository contains my personal implementation of the seminal Ray Tracing in One Weekend book, originally written by Peter Shirley.
The goal of this project is twofold:
Deepen my understanding of the Rust programming language by translating C++ concepts into idiomatic Rust (using traits, structs, ownership, and smart pointers like Arc and Box).
Learn the fundamental algorithms behind ray tracing and computer graphics.
## 🌟 Progress
This implementation covers the core concepts detailed in the book's chapters, including:
- [x] Basic vec3 (vector) and ray structures.
- [x] The camera model and simple viewpoint setup.
- [x] Defining a hittable abstraction (traits).
- [x] Implementing geometry: sphere and hittable_list.
- [ ] Materials and light interaction (lambertian, metal, dielectric).
- [ ] Multi-threading for fast rendering.
- [ ] Generating a complex scene with random spheres.

## 🛠️ Prerequisites
To build and run this project, you need to have the Rust toolchain installed.

## 🚀 Usage
### 1. Clone the Repository
git clone [https://github.com/MaxIvanyshen/rust-ray-tracing.git](https://github.com/MaxIvanyshen/rust-ray-tracing.git) \
cd rust-ray-tracing

### 2. Build the Project
This will compile the ray tracer executable.

```
cargo build --release
```

Note: We use the --release flag for compilation because ray tracing is computationally intensive, and the release build provides significant performance improvements.
### 3. Run the Renderer
The program is designed to output a single image file in the PPM format to standard output. We pipe this output to a file for viewing.

#### On Linux/macOS
```
cargo run --release > image.ppm
```

#### On Windows (using PowerShell)
```
cargo run --release | Out-File -Encoding ASCII image.ppm
```

### 4. View the Result
The PPM format is simple but not widely supported by default viewers. To view your rendered image (image.ppm), you can:
Use an online PPM viewer.
Use graphics software like IrfanView or GIMP.
Convert it to a more common format (like PNG or JPEG) using a tool like ImageMagick:
convert image.ppm image.png


## 💡 Learning & Code Structure
The core logic is structured to mirror the book's chapters:
src/main.rs: Handles command-line arguments, scene setup, and the final rendering loop.
src/vec3.rs: The fundamental vector and color structure.
src/ray.rs: The ray structure and its functions.
src/hittable.rs & src/sphere.rs: Defines the Hittable trait and geometry implementations.

Feel free to dive into the code! I've tried to leave comments explaining the core ray tracing math and how Rust concepts like Arc<dyn Hittable> manage polymorphic ownership.

## 📚 Acknowledgements
This project is a direct translation and learning exercise based on the original work:
Book: [Ray Tracing in One Weekend by Peter Shirley](https://raytracing.github.io/books/RayTracingInOneWeekend.html)

