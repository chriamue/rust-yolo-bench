# RustYoloBench

## Overview
`RustYoloBench` is an innovative project designed to benchmark machine learning frameworks in Rust, particularly focusing on running YOLO (You Only Look Once) models for real-time object detection. This repository compares popular Rust-based frameworks such as `tract`, `burn`, and `candle`, specifically with their capabilities to load and run ONNX models.

## Features
- **Framework Comparison**: Compare performance across different Rust-based machine learning frameworks.
- **ONNX Model Support**: Utilize ONNX models for object detection tasks.
- **Real-Time Analysis**: Process data from a live webcam feed using YOLO.
- **Interactive Web Frontend**: Built with Yew, the frontend allows users to select between frameworks and view the output in real time.
- **Performance Metrics**: Frame per second (FPS) rates are displayed for each framework to aid in performance comparison.

## Getting Started
To get started with `RustYoloBench`, you need to have Rust and the necessary dependencies installed on your system. Follow these steps:

1. **Clone the Repository**:

```
   git clone https://github.com/chriamue/rust-yolo-bench.git
```

2. **Install Dependencies**:

```bash
   cd rust-yolo-bench
   cargo install wasm-pack
   cargo install trunk
```
3. **Running the Application**:

```bash
    trunk serve
```
    This will start the application on `localhost:8080`.

## Run tests

```bash
wasm-pack test --firefox --headless
```

## Contributing
We welcome contributions to `RustYoloBench`! If you have suggestions or improvements, feel free to fork this repository and submit a pull request.

## License
This project is licensed under the [MIT License](LICENSE).
