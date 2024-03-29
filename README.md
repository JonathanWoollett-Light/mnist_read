# MNIST Read


[![Crates.io](https://img.shields.io/crates/v/mnist_read)](https://crates.io/crates/mnist_read)
[![lib.rs.io](https://img.shields.io/crates/v/mnist_read?color=blue&label=lib.rs)](https://lib.rs/crates/mnist_read)
[![docs](https://img.shields.io/crates/v/mnist_read?color=yellow&label=docs)](https://docs.rs/mnist_read)


Reads generic data and label files in the MNIST file format for Rust.

As simple as that.

```rust
// Raw format
let train_labels: Vec<u8> = mnist_read::read_labels("train-labels.idx1-ubyte");
let train_data: Vec<u8> = mnist_read::read_data("train-images.idx3-ubyte");

// Ndarray (Maths lib)
let usize_labels:Vec<usize> = train_labels.into_iter().map(|l|l as usize).collect();
let mut array_labels:ndarray::Array2<usize> = ndarray::Array::from_shape_vec((10000, 1), usize_labels).expect("Bad labels");

let f32_data:Vec<f32> = train_data.into_iter().map(|d|d as f32 / 255f32).collect();
let mut array_data:ndarray::Array2<f32> = ndarray::Array::from_shape_vec((10000, 28*28), f32_data).expect("Bad data");

// Cogent (Neural network library)
let mut net = cogent::NeuralNetwork::new(784,&[
    cogent::Layer::Dense(1000, cogent::Activation::ReLU),
    cogent::Layer::Dropout(0.2),
    cogent::Layer::Dense(500, cogent::Activation::ReLU),
    cogent::Layer::Dropout(0.2),
    cogent::Layer::Dense(10, cogent::Activation::Softmax)
])
net.train(&mut array_data, &mut array_labels).go()
```
