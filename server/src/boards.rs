use crate::{board_variables::EvaluateBoardVariable, config_manager::ConfigWrapper, matrix_server::helpers::{image_helper::draw_image, text_helpers::draw_text}, state_manager::StateWrapper};
use shared::{boards::{BoardDefinition, BoardElement, BoardElementValue, ElementColour}, device_config::{get_current_brightness, DeviceConfig}};

static DEBUG: bool = false;

pub trait BoardRender {
    async fn render(&self, device_config: &DeviceConfig, config:ConfigWrapper, state:StateWrapper) -> Option<String>;
}
impl BoardRender for BoardDefinition {
    async fn render(&self, device_config: &DeviceConfig, config: ConfigWrapper, state: StateWrapper) -> Option<String> {
        let current_brightness = get_current_brightness(&device_config.brightness);
        if self.use_skip_brightness_threshold && current_brightness < device_config.skip_brightness_threshold {
            return None;
        }
        let mut render_buffer = format!("b{:>03}======x=========cFFF======", current_brightness);
        // Send clear board when brightness is 0
        if current_brightness == 0 {
            return Some(render_buffer);
        }
        // Continue normally otherwise
        for board_element in &self.board_elements {
            render_buffer.push_str(&board_element.draw(config.clone(), state.clone(), device_config, &self.name).await);
        }
        return Some(render_buffer);
    }
}

pub trait DrawBoardElement {
    async fn draw(&self, config: ConfigWrapper, state: StateWrapper, device_config: &DeviceConfig, board_name: &str) -> String;
}
impl DrawBoardElement for BoardElement {
    async fn draw(&self, config: ConfigWrapper, state: StateWrapper, device_config: &DeviceConfig, board_name: &str) -> String {
        let legacy_mode = device_config.proto_version == 0;
        match self.value {
            BoardElementValue::Text(_) => {
                return draw_text(config.clone(), device_config, board_name, self.x, self.y, &self.colour, &self.font, self.value.substitute_variables(config.clone(), state.clone()).await).await;
            },
            BoardElementValue::Img(_,_) => {
                return draw_image(self.x, self.y, self.value.substitute_variables(config.clone(), state.clone()).await, legacy_mode, config.clone(), state.clone()).await;
            },
            BoardElementValue::Pixel => {
                let x = self.x.unwrap_or(0);
                let y = self.y;
                let colour = match self.colour {
                    shared::boards::ColourOption::Default => ElementColour::default(),
                    shared::boards::ColourOption::Specific(col) => col.clone(),
                    shared::boards::ColourOption::ParseTemperature => ElementColour::default(),
                };
                let mut colour = colour.to_string();
                colour = colour.replace('=',"");
                colour = colour.split_off(1);
                return format!("q{x:02}{y:02}{colour}==");
            },
            BoardElementValue::Line(x2, y2, _) => {
                let x = self.x.unwrap_or(0);
                let y = self.y;
                let temp_var = self.value.substitute_variables(config.clone(), state.clone()).await;
                let colour = match self.colour {
                    shared::boards::ColourOption::Default => ElementColour::default(),
                    shared::boards::ColourOption::Specific(col) => col.clone(),
                    shared::boards::ColourOption::ParseTemperature => {
                        let mut col = ElementColour::default();
                        let re = regex::Regex::new(r"(?P<temp>\d+)").unwrap();
                        let cap = re.captures(&temp_var);
                        if cap.is_some() {
                        let cap = cap.unwrap();
                            let temp = &cap["temp"];
                            if let Ok(temp) = temp.parse::<i32>() {
                                col = device_config.temperature_colours.get_colour(temp);
                            }
                        }
                        col
                    },
                };
                let colour = colour.to_string();
                return format!("{colour}l{x:02}{y:02}{x2:02}{y2:02}=");
            }
        }
    }
}

pub trait BoardElementValueSubstitute {
    async fn substitute_variables(&self, config: ConfigWrapper, state: StateWrapper) -> String;
}
impl BoardElementValueSubstitute for BoardElementValue {
    async fn substitute_variables(&self, config: ConfigWrapper, state: StateWrapper) -> String {
        match self {
            BoardElementValue::Text(x) => {
                if DEBUG {
                    tracing::info!("Substituting var '{}'", &x);
                }
                let config = config.read().await;
                let mut display_text = x.clone();
                for (key, val) in &config.board_variables {
                    if DEBUG {
                        tracing::info!("\tSubstituting  '__{}__'", key);
                    }
                    let key_match = format!("__{}__", key);
                    if display_text.contains(&key_match) {
                        display_text = display_text.replace(&key_match, &val.eval_variable(&key_match, &config, state.clone()).await);
                    }
                    if DEBUG {
                        tracing::info!("\t\tSubstitution Complete... New String: '{}'", &display_text);
                    }
                }
                return display_text;
            }
            BoardElementValue::Img(x, dynamic) => {
                if DEBUG {
                    tracing::info!("Substituting var '{}'", &x);
                }
                let config = config.read().await;
                let mut display_text = x.clone();
                if *dynamic {
                    for (key, val) in &config.board_variables {
                        if DEBUG {
                            tracing::info!("\tSubstituting  '__{}__'", key);
                        }
                        let key_match = format!("__{}__", key);
                        if display_text.contains(&key_match) {
                            display_text = display_text.replace(&key_match, &val.eval_variable(&key_match, &config, state.clone()).await);
                        }
                        if DEBUG {
                            tracing::info!("\t\tSubstitution Complete... New String: '{}'", &display_text);
                        }
                    }
                }
                return display_text;
            },
            BoardElementValue::Pixel => String::new(),
            BoardElementValue::Line(_, _, x) => {
                if DEBUG {
                    tracing::info!("Substituting var '{}'", &x);
                }
                let config = config.read().await;
                let mut display_text = x.clone();
                for (key, val) in &config.board_variables {
                    if DEBUG {
                        tracing::info!("\tSubstituting  '__{}__'", key);
                    }
                    let key_match = format!("__{}__", key);
                    if display_text.contains(&key_match) {
                        display_text = display_text.replace(&key_match, &val.eval_variable(&key_match, &config, state.clone()).await);
                    }
                    if DEBUG {
                        tracing::info!("\t\tSubstitution Complete... New String: '{}'", &display_text);
                    }
                }
                return display_text;
            },
        }
    }
}