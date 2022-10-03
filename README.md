# rust-ncnn

ncnn Rust API.

## Prequisition

### CMake >= 3.12

Rust cmake needs `--parallel` option thus CMake>=3.12 is complusory

```bash
$ pip install cmake --upgrade --user
```

### Clang >= 3.9

Rust bindgen uses `clang` to generate `bindings.rs` with `c_api.h`.

```bash
$ sudo apt install clang-3.9 libclang-3.9-dev
```

## Build

Static ncnn build from source:
```bash
$ cd rust-ncnn/
$ cargo run --example get_version
```

Use prebuilt ncnn:
```bash
$ export NCNN_DIR="/path/to/your/ncnn/lib"
```

## Linking

By default library uses dynamic linking on linux and static linking on windows.

To explicitly use static linking:
```bash
$ cargo build --example benchmark --features ncnn-bind/static
```

To explicitly use dynamic linking:
```bash
$ cargo build --example benchmark --features ncnn-bind/dynamic
```

## Vulkan

Build with Vulkan support (requires Vulkan SDK):
```bash
$ cargo build --example benchmark --features ncnn-bind/vulkan
```

## Run Examples and UnitTest

```bash
$ cargo test
$ cargo run --example get_version
$ cargo run --example benchmark --release
...
squeezenet.param 		 20 ms
squeezenet_int8.param 		 98 ms
mobilenet.param 		 35 ms
mobilenet_int8.param 		 112 ms
mobilenet_v2.param 		 25 ms
mobilenet_v3.param 		 20 ms
shufflenet.param 		 20 ms
shufflenet_v2.param 		 17 ms
mnasnet.param 		 23 ms
proxylessnasnet.param 		 26 ms
efficientnet_b0.param 		 37 ms
regnety_400m.param 		 27 ms
blazeface.param 		 6 ms
googlenet.param 		 85 ms
googlenet_int8.param 		 267 ms
resnet18.param 		 87 ms
resnet18_int8.param 		 349 ms
alexnet.param 		 114 ms
vgg16.param 		 277 ms
vgg16_int8.param 		 2029 ms
resnet50.param 		 141 ms
resnet50_int8.param 		 533 ms
squeezenet_ssd.param 		 149 ms
squeezenet_ssd_int8.param 		 252 ms
mobilenet_ssd.param 		 69 ms
mobilenet_ssd_int8.param 		 210 ms
mobilenet_yolo.param 		 160 ms
mobilenetv2_yolov3.param 		 73 ms
yolov4-tiny.param 		 103 ms
```
