use std::io;
use std::fs;
use std::vec;
use regex::Regex;

#[derive(Debug)]
struct Passport {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

impl Passport {
    fn valid_1(&self) -> bool  {
        !self.byr.is_empty() &&
            !self.iyr.is_empty() &&
            !self.eyr.is_empty() &&
            !self.hgt.is_empty() &&
            !self.hcl.is_empty() &&
            !self.ecl.is_empty() &&
            !self.pid.is_empty()
    }

    fn valid_2(&self) -> bool {
        self.valid_byr() &&
            self.valid_iyr() &&
            self.valid_eyr() &&
            self.valid_hgt() &&
            self.valid_hcl() &&
            self.valid_ecl() &&
            self.valid_pid() &&
            self.valid_cid()
    }

    fn valid_byr(&self) -> bool {
        match self.byr.parse::<u32>() {
            Ok(byr) => (1920..=2002).contains(&byr),
            _ => false
        }
    }

    fn valid_iyr(&self) -> bool {
        match self.iyr.parse::<u32>() {
            Ok(iyr) => (2010..=2020).contains(&iyr),
            _ => false
        }
    }

    fn valid_eyr(&self) -> bool {
        match self.eyr.parse::<u32>() {
            Ok(eyr) => (2020..=2030).contains(&eyr),
            _ => false
        }
    }

    fn valid_hgt(&self) -> bool {
        let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        if let Some(captures) = re.captures(&self.hgt) {
            return match captures.get(2).unwrap().as_str() {
                "cm" => (150..=193).contains(&captures.get(1).unwrap().as_str().parse::<u32>().unwrap()),
                "in" => (59..=76).contains(&captures.get(1).unwrap().as_str().parse::<u32>().unwrap()),
                _ => false
            }
        }
        false
    }

    fn valid_hcl(&self) -> bool {
        Regex::new(r"^#[0-9a-f]{6}$").unwrap().is_match(&self.hcl)
    }

    fn valid_ecl(&self) -> bool {
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&*self.ecl)
    }

    fn valid_pid(&self) -> bool {
        Regex::new(r"^[0-9]{9}$").unwrap().is_match(&self.pid)
    }

    fn valid_cid(&self) -> bool {
        true
    }
}

fn main() -> io::Result<()> {
    let passports = read_input("in-data/day4.txt")?;
    let part_1 = passports.iter().filter(|p| p.valid_1()).count();
    let part_2 = passports.iter().filter(|p| p.valid_2()).count();

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn read_input(filename: &str) -> io::Result<vec::Vec<Passport>> {
    let contents = fs::read_to_string(filename)?;
    let passport_data = contents.trim().split("\n\n").collect::<vec::Vec<&str>>();
    Ok(passport_data.iter().map(parse_passport).collect::<vec::Vec<Passport>>())
}

fn parse_passport(data: &&str) -> Passport {
    let mut passport = Passport {
        byr: "".to_string(),
        iyr: "".to_string(),
        eyr: "".to_string(),
        hgt: "".to_string(),
        hcl: "".to_string(),
        ecl: "".to_string(),
        pid: "".to_string(),
        cid: "".to_string(),
    };
    for field in  data.split(|c| c == '\n' || c == ' ') {
        let kv = field.split(':').collect::<vec::Vec<&str>>();
        match kv[0] {
            "byr" => passport.byr = kv[1].to_string(),
            "iyr" => passport.iyr = kv[1].to_string(),
            "eyr" => passport.eyr = kv[1].to_string(),
            "hgt" => passport.hgt = kv[1].to_string(),
            "hcl" => passport.hcl = kv[1].to_string(),
            "ecl" => passport.ecl = kv[1].to_string(),
            "pid" => passport.pid = kv[1].to_string(),
            "cid" => passport.cid = kv[1].to_string(),
            _ => println!("Unknown key: {}", kv[0])
        }
    }

    passport
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_2() {
        let pass = parse_passport(&"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm");
        assert!(pass.valid_2());

        let pass = parse_passport(&"iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929");
        assert!(! pass.valid_2());

        let pass = parse_passport(&"hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm");
        assert!(pass.valid_2());

        let pass = parse_passport(&"hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in");
        assert!(! pass.valid_2());
    }
}
