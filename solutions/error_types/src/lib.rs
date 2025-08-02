use chrono::Utc;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date:        String,
    pub err:         &'static str,
}

impl FormError {
    pub fn new(field: &'static str, value: String, err: &'static str) -> Self {
        Self {
            form_values: (field, value),
            date: Utc::now()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name:     String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new("name", self.name.clone(), "Username is empty"));
        }

        if self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long",
            ));
        }

        let (mut has_alpha, mut has_digit, mut has_sym) = (false, false, false);
        for c in self.password.chars() {
            if c.is_ascii_alphabetic()           { has_alpha = true; }
            else if c.is_ascii_digit()           { has_digit = true; }
            else if c.is_ascii_graphic()         { has_sym  = true; } // printable symbol
        }

        if has_alpha && has_digit && has_sym {
            Ok(())
        } else {
            Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols",
            ))
        }
    }
}
