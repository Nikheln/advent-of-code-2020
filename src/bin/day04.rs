use lazy_static::lazy_static;
use regex::Regex;

mod helpers;

fn main() {
    let filename: &str = "day04.txt";
    let input = helpers::input_helpers::read_input(&filename).unwrap();

    task_1(&input);
    task_2(&input);
}

fn task_1(input: &[String]) -> u32 {
    let result = parse_passports(input);

    // Check that all the fields are present (except cid)
    let valid_passport_count = result.iter().filter(|pp| pp.has_mandatory_fields()).count() as u32;

    println!("Task 1: {}", valid_passport_count);

    valid_passport_count
}

fn task_2(input: &[String]) -> u32 {
    let result = parse_passports(input);

    // Check that all the fields are valid
    let valid_passport_count = result.iter().filter(|pp| pp.is_valid()).count() as u32;

    println!("Task 2: {}", valid_passport_count);

    valid_passport_count
}

fn parse_passports(input: &[String]) -> Vec<Passport> {
    let mut passports = Vec::new();
    let re = Regex::new(r"(\w+):(\S+)").unwrap();

    let mut current_passport = Passport::new();
    for row in input {
        if row.is_empty() {
            passports.push(current_passport);
            current_passport = Passport::new();
        } else {
            for cap in re.captures_iter(row) {
                let value = Some(cap[2].to_string());
                match &cap[1] {
                    "byr" => current_passport.byr = value,
                    "iyr" => current_passport.iyr = value,
                    "eyr" => current_passport.eyr = value,
                    "hgt" => current_passport.hgt = value,
                    "hcl" => current_passport.hcl = value,
                    "ecl" => current_passport.ecl = value,
                    "pid" => current_passport.pid = value,
                    "cid" => current_passport.cid = value,
                    &_ => {}
                }
            }
        }
    }
    passports.push(current_passport);

    passports
}

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

lazy_static! {
    static ref HEIGHT_RE: Regex = Regex::new(r"(\d+)(\w\w)").unwrap();
    static ref HAIR_COLOR_RE: Regex = Regex::new(r"#[0-9a-f]{6}").unwrap();
    static ref PASSPORT_ID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

impl Passport {

    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    /*
     * Check that the passport has all the mandatory fields (i.e. all fields except `cid`).
     */
    fn has_mandatory_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    /*
     * Check that the passport has all the mandatory fields (i.e. all fields except `cid`)
     * and the values are valid.
     */
    fn is_valid(&self) -> bool {
        if !self.has_mandatory_fields() {
            return false;
        }

        match self.byr.as_ref().unwrap().parse::<u32>() {
            Ok(_byr @ 1920..=2002) => {} // No-op, field is valid
            _ => {
                return false;
            }
        }
        match self.iyr.as_ref().unwrap().parse::<u32>() {
            Ok(_iyr @ 2010..=2020) => {} // No-op, field is valid
            _ => {
                return false;
            }
        }
        match self.eyr.as_ref().unwrap().parse::<u32>() {
            Ok(_eyr @ 2020..=2030) => {} // No-op, field is valid
            _ => {
                return false;
            }
        }

        match HEIGHT_RE.captures(&self.hgt.as_ref().unwrap()) {
            Some(height_caps) => match height_caps[1].parse::<u32>() {
                Ok(height) => match &height_caps[2] {
                    "cm" if (150 <= height && height <= 193) => {}
                    "in" if (59 <= height && height <= 76) => {}
                    _ => {
                        return false;
                    }
                },
                _ => {
                    return false;
                }
            },
            None => {
                return false;
            }
        }

        if !HAIR_COLOR_RE.is_match(&self.hcl.as_ref().unwrap()) {
            return false;
        }

        let valid_eye_colors = [
            "amb".to_string(),
            "blu".to_string(),
            "brn".to_string(),
            "gry".to_string(),
            "grn".to_string(),
            "hzl".to_string(),
            "oth".to_string(),
        ];
        if !valid_eye_colors.contains(&self.ecl.as_ref().unwrap()) {
            return false;
        }

        if !PASSPORT_ID_RE.is_match(&self.pid.as_ref().unwrap()) {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn verify_example_task_1() {
        let input: Vec<String> = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
            "".to_string(),
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
            "hcl:#cfa07d byr:1929".to_string(),
            "".to_string(),
            "hcl:#ae17e1 iyr:2013".to_string(),
            "eyr:2024".to_string(),
            "ecl:brn pid:760753108 byr:1931".to_string(),
            "hgt:179cm".to_string(),
            "".to_string(),
            "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
            "iyr:2011 ecl:brn hgt:59in".to_string(),
        ];

        let result = crate::task_1(&input);
        assert_eq!(2, result);
    }

    #[test]
    fn verify_example_task_2_invalid() {
        let input: Vec<String> = vec![
            "eyr:1972 cid:100".to_string(),
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
            "".to_string(),
            "iyr:2019".to_string(),
            "hcl:#602927 eyr:1967 hgt:170cm".to_string(),
            "ecl:grn pid:012533040 byr:1946".to_string(),
            "".to_string(),
            "hcl:dab227 iyr:2012".to_string(),
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
            "".to_string(),
            "hgt:59cm ecl:zzz".to_string(),
            "eyr:2038 hcl:74454a iyr:2023".to_string(),
            "pid:3556412378 byr:2007".to_string(),
        ];

        let result = crate::task_2(&input);
        assert_eq!(0, result);
    }

    #[test]
    fn verify_example_task_2_valid() {
        let input: Vec<String> = vec![
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
            "hcl:#623a2f".to_string(),
            "".to_string(),
            "eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
            "".to_string(),
            "hcl:#888785".to_string(),
            "hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
            "pid:545766238 ecl:hzl".to_string(),
            "eyr:2022".to_string(),
            "".to_string(),
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
        ];

        let result = crate::task_2(&input);
        assert_eq!(4, result);
    }
}
