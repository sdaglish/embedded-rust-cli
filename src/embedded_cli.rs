use heapless::String;

pub struct EmbeddedCli {
    pub name: &'static str,
    command_string: String<32>,
    output_string: String<32>,
}

impl EmbeddedCli {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            command_string: String::new(),
            output_string: String::new(),
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.command_string.push(c).unwrap();
        self.output_string.push(c).unwrap();
        // let mut s = String::<32>::new();
        // s.push_str(self.name).unwrap();
        // s.push(c).unwrap();
        // rtt_target::rprintln!("{}", s);
    }

    // pub fn get_char(&mut self) -
    //     > Option<char> {
    //     self.output_string.pop()
    // }
}
