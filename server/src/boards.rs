use crate::{board_variables::EvaluateBoardVariable, config_manager::ConfigWrapper, matrix_server::helpers::{image_helper::draw_image, text_helpers::draw_text}, state_manager::StateWrapper};
use shared::{boards::{BoardDefinition, BoardElement, BoardElementValue}, device_config::{get_current_brightness, Brightnesses, DeviceConfig}};

static DEBUG: bool = false;

pub trait BoardRender {
    async fn render(&self, device_config: &DeviceConfig, config:ConfigWrapper, state:StateWrapper) -> String;
}
impl BoardRender for BoardDefinition {
    async fn render(&self, device_config: &DeviceConfig, config: ConfigWrapper, state: StateWrapper) -> String {
        let brightnesses: &Brightnesses = &device_config.brightness;
        let current_brightness = get_current_brightness(brightnesses);
        let mut render_buffer = format!("b{:>03}======x=========cFFF======", current_brightness);
        for board_element in &self.board_elements {
            render_buffer.push_str(&board_element.draw(config.clone(), state.clone(), &self.name).await);
        }
        return render_buffer;
    }
}

pub trait DrawBoardElement {
    async fn draw(&self, config: ConfigWrapper, state: StateWrapper, board_name: &str) -> String;
}
impl DrawBoardElement for BoardElement {
    async fn draw(&self, config: ConfigWrapper, state: StateWrapper, board_name: &str) -> String {
        match self.value {
            BoardElementValue::Text(_) => {
                return draw_text(config.clone(), board_name, self.x, self.y, &self.colour, &self.font, self.value.substitute_variables(config, state).await).await;
            },
            BoardElementValue::Img(_) => {
                return draw_image(self.x, self.y, self.value.substitute_variables(config, state).await).await;
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
            BoardElementValue::Img(x) => x.clone()
        }
    }
}