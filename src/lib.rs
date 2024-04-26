use rand::Rng;

pub struct Robot {
    name : String,
}

impl Robot {
    pub fn new() -> Self {
        let name = random_name();
        Robot { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = random_name()
    }
}

fn random_name() -> String{
    let mut rng = rand::thread_rng();
    let random_letter: String = (0..2).map(|_| rng.gen_range('A'..'Z') as char).collect();
    let random_number: u32 = rng.gen_range(1..1000);
    let formatted_number = format!("{:03}", random_number);
    format!("{}{}", random_letter, formatted_number)
}

//Still to implement checking of duplicate names