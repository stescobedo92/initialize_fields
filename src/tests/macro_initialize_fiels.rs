#[cfg(test)]
mod tests {
    use super::field;

    #[derive(Debug)]
    struct Example {
        #[field("Jhon")]
        name: String,

        #[field("Doe")]
        last_name: String,

        #[field("32")]
        age: i32,
    }

    #[test]
    fn test_initializer_macro() {
        let expected = Example {
            name: "Jhon".into(),
            last_name: "Doe".into(),
            age: 32,
        };

        let example: Example = Example::new();
        assert_eq!(example, expected);
    }

    #[test]
    #[should_panic(expected = "Field 'name' is missing #[field(...)] attribute")]
    fn test_missing_field_attribute() {
        let expected = Example {
            name: "Jhon".into(),
            last_name: "Doe".into(),
            age: 36,
        };

        let example: Example = Example::new();
        assert_eq!(example, expected);
    }
}