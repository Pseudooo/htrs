use clap::ArgMatches;

pub trait MatchBinding<T> {
    fn bind_field(&self, field_id: &str) -> T;
}

impl MatchBinding<String> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> String {
        let Some(field_value) = self.get_one::<String>(field_id) else {
            panic!("Unexpected binding - no value found");
        };
        field_value.clone()
    }
}

impl MatchBinding<Option<String>> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> Option<String> {
        let Some(value) = self.get_one::<String>(field_id) else {
            return None
        };
        Some(value.clone())
    }
}

impl MatchBinding<bool> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> bool {
        self.get_flag(field_id)
    }
}

impl MatchBinding<Option<bool>> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> Option<bool> {
        let Some(value) = self.get_one::<bool>(field_id) else {
            return None;
        };

        Some(*value)
    }
}

impl MatchBinding<Vec<String>> for ArgMatches {
    fn bind_field(&self, field_id: &str) -> Vec<String> {
        let binding = self.get_many::<String>(field_id);
        let Some(binding_value) = binding else {
            return vec![];
        };
        binding_value.cloned().collect()
    }
}
