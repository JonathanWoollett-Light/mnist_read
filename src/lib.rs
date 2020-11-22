//! Reads generic data and label files in the MNIST format.

use std::{fs::File, io::Read};

/// Reads MNIST format labels.
///
/// Returns `Vec<usize>` (length = number of labels).
/// ```
/// let test_labels: Vec<u8> = mnist_read::read_labels("mnist/t10k-labels.idx1-ubyte");
///
/// assert_eq!(test_labels.len(),10000);
///
/// for label in test_labels {
///     assert!(label <= 9)
/// }
/// ```
/// For [ndarray](https://docs.rs/ndarray/) one might do:
/// ```
/// let u8_labels:Vec<u8> = mnist_read::read_labels("mnist/t10k-labels.idx1-ubyte");
/// let usize_labels:Vec<usize> = u8_labels.into_iter().map(|l|l as usize).collect();
/// let array_labels:ndarray::Array2<usize> = ndarray::Array::from_shape_vec((10000, 1), usize_labels).expect("Bad labels");
/// assert_eq!(array_labels.shape(),&[10000,1]);
/// ```
pub fn read_labels(path: &str) -> Vec<u8> {
    // Reads file
    let mut file = File::open(path).unwrap();
    let mut label_buffer_u8: Vec<u8> = Vec::new();
    file.read_to_end(&mut label_buffer_u8)
        .expect("Couldn't read MNIST labels");

    // Removes 1st 7 bytes of meta data & returns
    return label_buffer_u8.drain(8..).collect();
}
/// Reads MNIST format data.
///
/// Returns `Vec<u8>` (length = number of examples * size of examples).
/// ```
/// let test_data: Vec<u8> = mnist_read::read_data("mnist/t10k-images.idx3-ubyte");
///
/// assert_eq!(test_data.len(),10000*28*28);
/// ```
/// For [ndarray](https://docs.rs/ndarray/) one might do:
/// ```
/// let u8_data:Vec<u8> = mnist_read::read_data("mnist/t10k-images.idx3-ubyte");
/// let f32_data:Vec<f32> = u8_data.into_iter().map(|d|d as f32 / 255f32).collect();
/// let array_data:ndarray::Array2<f32> = ndarray::Array::from_shape_vec((10000, 28*28), f32_data).expect("Bad data");
/// assert_eq!(array_data.shape(),&[10000,28*28]);
/// ```
pub fn read_data(path: &str) -> Vec<u8> {
    // Reads file
    let mut file = File::open(path).unwrap();
    let mut image_buffer_u8: Vec<u8> = Vec::new();
    file.read_to_end(&mut image_buffer_u8)
        .expect("Couldn't read MNIST data");
    // Removes 1st 16 bytes of meta data & returns
    return image_buffer_u8.drain(16..).collect();
}
