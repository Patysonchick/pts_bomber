#[derive(Debug, Clone)]
pub struct Phone {
    pub phone: String,
    pub country: Country,
}

#[derive(Debug, Clone)]
pub enum Country {
    Ru,
}

#[allow(dead_code)]
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum FormatterTypes {
    WithPlus,
    WithPlusHyphen,         // +7 *** ***-**-**
    WithPlusBracketsHyphen, // +7 (***) ***-**-**
}

#[derive(Debug)]
pub enum FormatterErrors {
    IncorrectPatter,
    IncorrectLength,
}

impl Phone {
    pub fn new(raw_phone: String, country: Country) -> Result<Self, FormatterErrors> {
        let mut phone = String::new();
        let numbers: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        for segment in raw_phone.chars() {
            for number in numbers {
                if segment == number {
                    phone.push(segment);
                }
            }
        }

        let result = Self { phone, country };
        match result.check_phone() {
            Ok(_) => Ok(result),
            Err(e) => Err(e),
        }
    }

    fn check_phone(&self) -> Result<(), FormatterErrors> {
        match self.country {
            Country::Ru => {
                if &self.phone[0..2] != "79" && &self.phone[0..2] != "74" {
                    Err(FormatterErrors::IncorrectPatter)
                } else if self.phone.len() != 11 {
                    Err(FormatterErrors::IncorrectLength)
                } else {
                    Ok(())
                }
            }
        }
    }

    pub fn format(&mut self, formatter: FormatterTypes) {
        match self.country {
            Country::Ru => match formatter {
                FormatterTypes::WithPlus => {
                    let mut formatted = String::new();

                    formatted.push('+');
                    formatted.push_str(&self.phone[0..11]);

                    self.phone = formatted;
                }
                FormatterTypes::WithPlusHyphen => {
                    let mut formatted = String::new();

                    formatted.push('+');
                    formatted.push(self.phone.chars().nth(0).unwrap());
                    formatted.push(' ');
                    formatted.push_str(&self.phone[1..4]);
                    formatted.push(' ');
                    formatted.push_str(&self.phone[4..7]);
                    formatted.push('-');
                    formatted.push_str(&self.phone[7..9]);
                    formatted.push('-');
                    formatted.push_str(&self.phone[9..11]);

                    self.phone = formatted;
                }
                FormatterTypes::WithPlusBracketsHyphen => {
                    let mut formatted = String::new();

                    formatted.push('+');
                    formatted.push(self.phone.chars().nth(0).unwrap());
                    formatted.push(' ');
                    formatted.push('(');
                    formatted.push_str(&self.phone[1..4]);
                    formatted.push(')');
                    formatted.push(' ');
                    formatted.push_str(&self.phone[4..7]);
                    formatted.push('-');
                    formatted.push_str(&self.phone[7..9]);
                    formatted.push('-');
                    formatted.push_str(&self.phone[9..11]);

                    self.phone = formatted;
                }
            },
        }
    }
}
