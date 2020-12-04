mod passport;

use std::fs;
use std::result::Result::Ok;

fn main() -> anyhow::Result<()> {
    let options = common::simple_cli::Opts::get();

    let content = fs::read_to_string(options.input)?;

    let batch = passport::Batch::from(content.as_str());

    let required_fields = get_requirements();

    // Part 1
    let part1_count = batch
        .get_passports_containing_fields(&required_fields)
        .len();

    println!("Part 1: {} valid passports", part1_count);

    // Part 2
    let part2_count = batch
        .get_passports_containing_fields(&required_fields)
        .iter()
        .filter(|p| is_valid_passport(p, &required_fields))
        .count();

    println!("Part 2: {} valid passports", part2_count);

    Ok(())
}

fn get_requirements() -> [passport::PassportFieldParse<'static>; 7] {
    use passport::PassportFieldParse::*;

    [
        BirthYear,
        IssueYear,
        ExpirationYear,
        Height,
        HairColor,
        EyeColor,
        PassportID,
    ]
}

fn is_valid_passport(
    passport: &passport::PassportParse,
    fields: &[passport::PassportFieldParse],
) -> bool {
    passport
        .fields
        .iter()
        .filter(|(t, _)| fields.contains(t)) // Only check required fields
        .all(|field| is_valid_field(field)) // all remaining fields have to be valid
}

fn is_valid_field(field: &(passport::PassportFieldParse, &str)) -> bool {
    use passport::PassportFieldParse::*;

    // use specified method for valid checking
    match field {
        (BirthYear, value) => validate_year(value, 1920, 2002),
        (IssueYear, value) => validate_year(value, 2010, 2020),
        (ExpirationYear, value) => validate_year(value, 2020, 2030),
        (Height, value) => validate_height(value),
        (HairColor, value) => validate_hair_color(value),
        (EyeColor, value) => validate_eye_color(value),
        (PassportID, value) => validate_passport_id(value),
        _ => unreachable!("CountryID and Unknown should have been filtered out"),
    }
}

// parse as int and check if between given values
fn validate_year(value: &str, bigger_or_equal: usize, smaller_or_equal: usize) -> bool {
    let year = if let Ok(year) = value.parse::<usize>() {
        year
    } else {
        return false;
    };

    bigger_or_equal <= year && year <= smaller_or_equal
}

// check for suffix cm or in, strip it, parse rest as int and check
fn validate_height(value: &str) -> bool {
    if let Some(stripped) = value.strip_suffix("cm") {
        if let Ok(height_cm) = stripped.parse::<usize>() {
            150 <= height_cm && height_cm <= 193
        } else {
            false
        }
    } else if let Some(stripped) = value.strip_suffix("in") {
        if let Ok(height_in) = stripped.parse::<usize>() {
            59 <= height_in && height_in <= 76
        } else {
            false
        }
    } else {
        false
    }
}

// check for prefix #, strip it, filter remaining chars if hex component and count fitting, should be 6
fn validate_hair_color(value: &str) -> bool {
    if let Some(without_pound) = value.strip_prefix('#') {
        without_pound
            .chars()
            .filter(|c| char_is_hex_component(c))
            .count()
            == 6
    } else {
        false
    }
}
fn char_is_hex_component(char: &char) -> bool {
    matches!(char, '0'..='9' | 'a'..='f')
}

// check if value is one of specified
fn validate_eye_color(value: &str) -> bool {
    matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

// filter chars of value for numerical chars and count remaining, should be 9
fn validate_passport_id(value: &str) -> bool {
    value.chars().filter(|c| char_is_number(c)).count() == 9
}

fn char_is_number(char: &char) -> bool {
    matches!(char, '0'..='9')
}
