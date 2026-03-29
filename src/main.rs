//imports
use rand;

use iced::widget::{Column, Container, Row, button, column, container, row, text, text_input, toggler, TextInput};
use iced::{Alignment::Center, alignment::Horizontal::Left};
use iced::{Theme, Length};
use iced_aw::widget::{number_input};

//
#[derive(Default)]
struct Characters{
    capital_letters: [char; 26],
    lowercase_letters: [char; 26],
    numbers: [char; 10],
    special_characters: [char; 8],

}

//Password Generator Struct
#[derive(Default)]
struct PasswordGenerator{
    length: u32,
    max_length: u32,
    special_characters_enabled: bool,
    capital_letters_enabled: bool,
    numbers_enabled: bool,
    password: String,
    characters: Characters,
}

//Implementation of Characters
impl Characters{
    fn default() -> Characters{
        let mut data = Characters{
            capital_letters: ['A'; 26],
            lowercase_letters: ['a'; 26],
            numbers: ['0';10],
            special_characters: ['!', '@', '#', '$', '%', '^', '&', '*'],
        };
        for i in 0..26{
            data.capital_letters[i] = (b'A'+ i as u8) as char;
            data.lowercase_letters[i] = (b'a' + i as u8) as char;
        }
        for i in 0..10{
            data.numbers[i] = (b'0' + i as u8) as char;
        }
        data
    }

    fn enabled_characters(&mut self, capital_letters_enabled: bool, numbers_enabled: bool, special_characters_enabled: bool) -> String{
        let mut output = String::new();
        for i in self.lowercase_letters{
            output.push(i);
        }
        if capital_letters_enabled{
            for i in self.capital_letters{
                output.push(i);
            }
        }
        if numbers_enabled{
            for i in self.numbers{
                output.push(i);
            }
        }
        if special_characters_enabled{
            for i in self.special_characters{
                output.push(i);
            }
        }
        return output
    }
}

//enum for Iced application messages
#[derive(Clone)]
enum Message {
    Submit,
    UpdateLength(u32),
    UpdateCapitalLetters(bool),
    UpdateSpecialCharacters(bool),
    UpdateNumbers(bool),
}


//Implementation of PasswordGenerator
impl PasswordGenerator{
    fn default() -> PasswordGenerator{
        let mut password_generator = PasswordGenerator {
            length: 8,
            max_length: 20,
            special_characters_enabled: true,
            capital_letters_enabled: true,
            numbers_enabled: true,
            password: String::new(),
            characters: Characters::default(),
        };
        password_generator
    }

    fn generate_password(&mut self, capital_letters_enabled: bool, numbers_enabled: bool, special_characters_enabled: bool) -> String{
        let mut password = String::new();
        let characters = self.characters.enabled_characters(capital_letters_enabled, numbers_enabled, special_characters_enabled);
        for i in 0..self.length{
            let random_index = rand::random_range(0..(characters.len()-1));
            let character = &characters[random_index..random_index+1];
            password.push_str(character);
        }
        password
    }

    fn update(&mut self, message: Message)
    {
        match message {
            Message::Submit => self.password = self.generate_password(self.capital_letters_enabled, self.numbers_enabled, self.special_characters_enabled),
            Message::UpdateLength(length) => self.length = length,
            Message::UpdateCapitalLetters(capital_letters_enabled) => {self.capital_letters_enabled = capital_letters_enabled;println!("{}", self.capital_letters_enabled);},
            Message::UpdateSpecialCharacters(special_characters_enabled) => self.special_characters_enabled = special_characters_enabled,
            Message::UpdateNumbers(numbers_enabled) => self.numbers_enabled = numbers_enabled,
        }
    }

    fn view(&self) -> Container<Message> {
        let submit = button("Submit")
            .on_press(Message::Submit);

        let toggle_capital_letters = toggler(self.capital_letters_enabled)
            .label("Capital Letters?")
            .on_toggle(Message::UpdateCapitalLetters);

        let toggle_special_characters = toggler(self.special_characters_enabled)
            .label("Special Characters?")
            .on_toggle(Message::UpdateSpecialCharacters);

        let toggle_numbers= toggler(self.numbers_enabled)
            .label("Numbers?")
            .on_toggle(Message::UpdateNumbers);

        let length_input = number_input(&self.length, 8..=self.max_length, Message::UpdateLength)
            .step(1);

        let password_output = text_input("", &self.password);
            //.on_input(Message::DoNotUpdatePassword);
            //.into();

        let togglers = column![
            toggle_capital_letters,
            toggle_special_characters,
            toggle_numbers
        ]
            .align_x(Left)
            .spacing(10);

        let interface = column![
            length_input,
            togglers,
            submit,
            password_output,
        ]
            .align_x(Center)
            .spacing(10);

        container(interface)
            .padding(10)
            .center_x(Length::Fill)
            .align_x(Center)
            .into()

        //interface
    }
}


//main
fn main() -> iced::Result {
    iced::application(PasswordGenerator::default, PasswordGenerator::update, PasswordGenerator::view)
    .theme(Theme::Dark)
    .run()
}
