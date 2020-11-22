#[cfg(test)]
mod tests {
    #[test]
    fn training_labels() {
        let training_labels: Vec<u8> = mnist_read::read_labels("mnist/train-labels.idx1-ubyte");
        assert_eq!(training_labels.len(), 60000);
        for label in training_labels {
            assert!(label <= 9)
        }
    }
    #[test]
    fn training_data() {
        let training_data: Vec<u8> = mnist_read::read_data("mnist/train-images.idx3-ubyte");
        assert_eq!(training_data.len(), 60000 * 28 * 28);
    }
}
