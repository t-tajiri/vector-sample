use vector_sample::SampleVector;

fn main() {
    let mut vector = SampleVector::new();

    vector.push("Java Finch".to_string());
    vector.push("Budgerigar".to_string());

    let element = vector.get(1);
    assert_eq!(element, Some(&"Budgerigar".to_string()));
}
