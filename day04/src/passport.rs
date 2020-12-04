use std::collections::HashSet;
use std::convert::From;

// Batch
//##################

pub struct Batch<'a> {
    passports: Vec<PassportParse<'a>>,
}

impl<'a> Batch<'a> {
    pub fn get_passports_containing_fields(
        &'a self,
        requirements: &[PassportFieldParse],
    ) -> Vec<&PassportParse<'a>> {
        self.passports
            .iter()
            .filter(|p| p.contain_fields(requirements))
            .collect()
    }
}

impl<'a> From<&'a str> for Batch<'a> {
    fn from(s: &'a str) -> Self {
        let passports = s.split("\n\n").map(PassportParse::from).collect::<Vec<_>>();
        Self { passports }
    }
}

//##################

// PassportParse
//##################

pub struct PassportParse<'a> {
    pub fields: Vec<(PassportFieldParse<'a>, &'a str)>,
}

impl<'a> PassportParse<'a> {
    pub fn contain_fields(&'a self, requirements: &[PassportFieldParse]) -> bool {
        // lazy approach: collect to hashset and intersect.
        // if intersection contains as many fields as requirements, then all required fields are there

        let requirement_set = requirements.iter().collect::<HashSet<_>>();

        let passport_set = self.fields.iter().map(|f| &f.0).collect::<HashSet<_>>();

        requirement_set.intersection(&passport_set).count() == requirements.len()
    }
}

impl<'a> From<&'a str> for PassportParse<'a> {
    fn from(s: &'a str) -> Self {
        let fields = s
            .split_whitespace() //split by whitespace -> multiple ID:VALUE fields
            .filter_map(|f| {
                let mut id_value = f.split(':'); //Split ID:VALUE Fields

                //Parse ID
                let id = if let Some(id) = id_value.next() {
                    PassportFieldParse::from(id)
                } else {
                    return None;
                };

                //Get Value
                let value = id_value.next()?;

                Some((id, value))
            })
            .collect::<Vec<_>>();

        Self { fields }
    }
}

//##################

// PassportFieldParse
//##################

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub enum PassportFieldParse<'a> {
    BirthYear,
    IssueYear,
    ExpirationYear,
    Height,
    HairColor,
    EyeColor,
    PassportID,
    CountryID,
    Unknown(&'a str),
}

impl<'a> From<&'a str> for PassportFieldParse<'a> {
    fn from(s: &'a str) -> Self {
        use PassportFieldParse::*;

        match s {
            "byr" => BirthYear,
            "iyr" => IssueYear,
            "eyr" => ExpirationYear,
            "hgt" => Height,
            "hcl" => HairColor,
            "ecl" => EyeColor,
            "pid" => PassportID,
            "cid" => CountryID,
            unknown => Unknown(unknown),
        }
    }
}

//##################
