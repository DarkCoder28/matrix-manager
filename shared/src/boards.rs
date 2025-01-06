use derive_builder::Builder;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BoardDefinition {
    pub name: String,
    pub size: (u8, u8),
    pub board_elements: Vec<BoardElement>,
    #[serde(skip_serializing_if = "is_false", default)]
    pub use_skip_brightness_threshold: bool,
}
impl Default for BoardDefinition {
    fn default() -> Self {
        BoardDefinition {
            name: String::from("clock"),
            size: (64, 32),
            board_elements: vec![
                BoardElementBuilder::default()
                    .name(String::from("Weekday"))
                    .y(0)
                    .value(BoardElementValue::Text(String::from("__weekday__")))
                    .build()
                    .unwrap(),
                BoardElementBuilder::default()
                    .name(String::from("Time"))
                    .y(9)
                    .font(Some(String::from("7x14B")))
                    .value(BoardElementValue::Text(String::from("__time__")))
                    .build()
                    .unwrap(),
                BoardElementBuilder::default()
                    .name(String::from("Date"))
                    .y(24)
                    .value(BoardElementValue::Text(String::from("__date__")))
                    .build()
                    .unwrap(),
            ],
            use_skip_brightness_threshold: false,
        }
    }
}

fn is_false(b: &bool) -> bool { !b }

#[derive(Serialize, Deserialize, Builder, Clone, PartialOrd, Debug)]
#[builder(default)]
pub struct BoardElement {
    pub name: String,
    pub x: Option<u8>,
    pub y: u8,
    pub colour: ColourOption,
    pub font: Option<String>,
    pub value: BoardElementValue,
}
impl PartialEq for BoardElement {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.x == other.x
            && self.y == other.y
            && self.colour == other.colour
            && self.font == other.font
            && self.value == other.value
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
impl BoardElement {
    pub fn set(&mut self, new_values: BoardElement) {
        self.name = new_values.name;
        self.x = new_values.x;
        self.y = new_values.y;
        self.colour = new_values.colour;
        self.font = new_values.font;
        self.value = new_values.value;
    }
}
impl Default for BoardElement {
    fn default() -> Self {
        let mut name = String::from("New Element - ");
        name.push_str(&u32::to_string(&get_rand()));
        BoardElement {
            name: name,
            x: None,
            y: 0,
            colour: ColourOption::Default,
            font: None,
            value: BoardElementValue::default(),
        }
    }
}
fn get_rand() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum ColourOption {
    Default,
    Specific(ElementColour),
    ParseTemperature,
}
impl Default for ColourOption {
    fn default() -> Self {
        Self::Default
    }
}
impl ColourOption {
    pub fn get_option(&self) -> String {
        match self {
            ColourOption::Default => ColourOption::get_options()[0].clone(),
            ColourOption::Specific(_) => ColourOption::get_options()[1].clone(),
            ColourOption::ParseTemperature => ColourOption::get_options()[2].clone(),
        }
    }
    pub fn get_options() -> Vec<String> {
        "Default;Specific;Parse Temperature".split(";").map(|x|x.to_string()).collect()
    }
    pub fn from_str(type_str: &str) -> ColourOption {
        match type_str {
            "Default" => ColourOption::Default,
            "Specific" => ColourOption::Specific(
                ElementColour::default(),
            ),
            "Parse Temperature" => {
                ColourOption::ParseTemperature
            }
            _ => ColourOption::Default,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct ElementColour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Default for ElementColour {
    fn default() -> Self {
        ElementColour {
            r: 0xFF,
            g: 0xFF,
            b: 0xFF,
            a: 0xFF,
        }
    }
}
impl ToString for ElementColour {
    fn to_string(&self) -> String {
        let r = self.r * (self.a / 0xFF);
        let g = self.g / (self.a / 0xFF);
        let b = self.b / (self.a / 0xFF);
        format!("c{:X}{:X}{:X}======", r >> 4, g >> 4, b >> 4)
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, PartialOrd, Debug)]
pub enum BoardElementValue {
    Text(String),
    Img(String, bool /* dynamic */),
    Pixel,
    Line(u8, u8, String),
}
impl Default for BoardElementValue {
    fn default() -> Self {
        BoardElementValue::Text(String::new())
    }
}

impl BoardElementValue {
    pub fn extract_element_value(&self) -> (String, String) {
        match self {
            BoardElementValue::Text(x) => (String::from("Text"), x.clone()),
            BoardElementValue::Img(x, _) => (String::from("Image"), x.clone()),
            BoardElementValue::Pixel => (String::from("Pixel"), String::new()),
            BoardElementValue::Line(_, _, x) => (String::from("Line"), x.clone()),
        }
    }
    pub fn get_type(&self) -> String {
        match self {
            BoardElementValue::Text(_) => String::from("Text"),
            BoardElementValue::Img(_, _) => String::from("Image"),
            BoardElementValue::Pixel => String::from("Pixel"),
            BoardElementValue::Line(_, _, _) => String::from("Line"),
        }
    }
    pub fn get_types() -> Vec<String> {
        "Text;Image;Pixel;Line".split(';').map(|x|x.to_string()).collect()
    }
    pub fn from_strings(type_string: &str, value: String, dynamic_img: bool) -> BoardElementValue {
        match type_string {
            "Text" => BoardElementValue::Text(value),
            "Image" => BoardElementValue::Img(value, dynamic_img),
            "Pixel" => BoardElementValue::Pixel,
            "Line" => BoardElementValue::Line(0, 0, value),
            _ => BoardElementValue::Text(value),
        }
    }
}
