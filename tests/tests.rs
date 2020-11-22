#[cfg(test)]
mod tests {
    #[test]
    fn training_labels() {
        let training_labels: Vec<u8> = mnist_read::read_labels("mnist/train-labels.idx1-ubyte");
        assert_eq!(training_labels.len(),60000);
        for label in training_labels {
            assert!(label <= 9)
        }
    }
    #[test]
    fn training_data() {
        let training_data: Vec<u8> = mnist_read::read_data("mnist/train-images.idx3-ubyte");
        assert_eq!(training_data.len(),60000*28*28);
    }
    // #[test]
    // fn holder_test() {
    //     let u8_labels:Vec<u8> = mnist_read::read_data("mnist/t10k-labels.idx1-ubyte");
    //     println!("u8_labels.len(): {}",u8_labels.len());
    //     let usize_labels:Vec<usize> = u8_labels.into_iter().map(|l|l as usize).collect();
    //     println!("usize_labels.len(): {}",usize_labels.len());
    //     let array_labels:ndarray::Array2<usize> = ndarray::Array::from_shape_vec((10000, 1), usize_labels).expect("Bad labels");
    // }
}