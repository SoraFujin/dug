pub fn validate_input(input: &str) -> Result<(), String> {
    let input = input.trim();
    // First Check: Make sure it has a dot in it
    if !input.contains(".") {
        return Err("Invalid domain: missing TLD (e.g. use \"google.com\" not \"google\")".to_string())
    }
    // Second Check: See if the domain name is larger than 253
    if input.len() > 253 {
        return Err("Invalid domain: exceeds maximum length of 253 characters".to_string())
    }
    // split at the dots
    for label in input.split(".") {
        // Check and validate each part alone

        // Third check: is to see if there are any empty strings (google..com)
        if label.is_empty() {
            return Err("Invalid domain: empty label (consecutive dots are not allowed)".to_string());
        }

        // Fourth check: See if each label does not exceed 63 characters
        if label.len() > 63 {
            return Err(format!("Invalid label \"{}\": exceeds maximum length of 63 characters", label));
        }

        // Fifth check: labels may only contain letters, digits, and hyphens
        let first = label.chars().next().unwrap();
        let last = label.chars().last().unwrap();

        if first == '-' || last == '-' {
            return Err(format!("Invalid label \"{}\": labels cannot start or end with a hyphen", label))
        }

        for c in label.chars() {
            if !c.is_alphanumeric() && c != '-' {
                return Err(format!("Invalid label \"{}\": invalid character '{}', only letters, digits, and hyphens are allowed", label, c))
            }
        }
    }

    Ok(())
}
