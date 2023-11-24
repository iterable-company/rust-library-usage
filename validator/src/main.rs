use derive_new::new;
use serde_json::Value;
use validator::{Validate, ValidationError, ValidationErrors, ValidationErrorsKind};

fn main() {
    // Player::name
    let invalid_name_player = Player::new("".to_string());
    match invalid_name_player.validate() {
        Ok(_) => {
            println!("valid player");
        }
        Err(e) => e
            .field_errors()
            .values()
            .flat_map(|vec| vec.into_iter())
            .for_each(|e| {
                println!(
                    "code: {}, message: {:?}, value: {:?}, misc: {:?}",
                    e.code, e.message, e.params["value"], e.params
                )
            }),
    };
    // Team::players
    let team_including_invalid_player = Team::new(vec![invalid_name_player], "{}".to_string());
    match team_including_invalid_player.validate() {
        Ok(_) => {
            println!("valid team");
        }
        Err(e) => extract_errors(&e).into_iter().for_each(|e| {
            println!(
                "code: {}, message: {:?}, value: {:?}, misc: {:?}",
                e.code, e.message, e.params["value"], e.params
            )
        }),
    };
    // Team::properties
    let team_including_invalid_player =
        Team::new(vec![Player::new("name".to_string())], "{".to_string());
    match team_including_invalid_player.validate() {
        Ok(_) => {
            println!("valid team");
        }
        Err(e) => extract_errors(&e).into_iter().for_each(|e| {
            println!(
                "code: {}, message: {:?}, value: {:?}, misc: {:?}",
                e.code, e.message, e.params["value"], e.params
            )
        }),
    };
}

fn extract_errors(errors: &ValidationErrors) -> Vec<&ValidationError> {
    let mut extracted_errors = Vec::new();

    for error in errors.errors().values() {
        match error {
            ValidationErrorsKind::Struct(s) => {
                extracted_errors.extend(extract_errors(s));
            }
            ValidationErrorsKind::List(l) => {
                for err in l.iter() {
                    extracted_errors.extend(extract_errors(err.1));
                }
            }
            ValidationErrorsKind::Field(f) => {
                extracted_errors.extend(f.iter());
            }
        }
    }

    extracted_errors
}

#[derive(Validate, new)]
struct Player {
    #[validate(length(min = 1, max = 100, code = "NAME", message = "invalid name"))]
    name: String,
}

#[derive(Validate, new)]
struct Team {
    #[validate]
    players: Vec<Player>,
    #[validate(custom = "validate_json")]
    properties: String,
}

fn validate_json(json_str: &str) -> Result<(), ValidationError> {
    match serde_json::from_str::<Value>(json_str) {
        Ok(_) => (),
        Err(_e) => {
            return Err(ValidationError::new("JSON"));
        }
    }
    Ok(())
}
