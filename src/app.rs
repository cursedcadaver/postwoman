use std::collections::HashMap;
pub enum CurrentScreen {
    Main,
    Get,
    Post,
    Exiting,
}
pub enum CurrentlyEditing {
    Key,
    Value,
}

struct App {
    pub key_input: String,
    pub value_input: String,
    pub url: String,
    pub pairs: HashMap<String, String>,
    pub current_screen: CurrentScreen,
    pub currently_editing: Option<CurrentlyEditing>, // t
}
impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            url: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }
    pub async fn post_req(&self) -> Result<(), Box<dyn std::error::Error>> {
        let output = serde_json::to_string(&self.pairs)?;
        let client = reqwest::Client::new();
        let res = client.post(&self.url).json(&output).send().await?;

        Ok(())
    }
    pub async fn get_req(&self) -> Result<(), Box<dyn std::error::Error>> {
        let body = reqwest::get(&self.url).await?.text().await?;
        Ok(())
    }
}
