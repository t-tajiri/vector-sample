use vector_sample::SampleVector;

fn main() {
    let mut vector = SampleVector::new();

    vector.push("Java Finch".to_string());
    vector.push("Budgerigar".to_string());

    let element = vector.get(1);
    assert_eq!(element, Some(&"Budgerigar".to_string()));

    let expect_default_message = &"default message".to_string();
    assert_eq!(
        expect_default_message,
        vector.get_or(2, expect_default_message)
    );

    assert_eq!(Some("Budgerigar".to_string()), vector.pop());
}
