use ini::Ini;

pub struct ConfigFile {
    content: Option<Ini>,
}

impl ConfigFile {
    pub fn new() -> ConfigFile {
        ConfigFile { content: None }
    }

    pub fn load(&mut self, content: &str) {
        self.content = Some(Ini::load_from_str(content).unwrap());
    }

    pub fn get(&self, section: &str, key: &str) -> Option<String> {
        let mut result = None;
        if let Some(ref ini) = self.content {
            if let Some(s) = ini.section(Some(section)) {
                if let Some(value) = s.get(key) {
                    result = Some(value.to_string())
                }
            }
        }
        result
    }
    pub fn set(&mut self, section: &str, key: &str, value: &str) -> Result<(), String> {
        match &mut self.content {
            Some(ini) => {
                ini.with_section(Some(section)).set(key, value);
                Ok(())
            }
            None => Err("Configuration not loaded".to_string()),
        }
    }
    pub fn add(&mut self, section: &str, key: &str, value: &str) -> Result<(), String> {
        match &mut self.content {
            Some(ini) => {
                ini.with_section(Some(section)).add(key, value);
                Ok(())
            }
            None => Err("Configuration not loaded".to_string()),
        }
    }
    pub fn delete(&mut self, section: &str, key: &str) -> Result<(), String> {
        match &mut self.content {
            Some(ini) => {
                ini.delete_from(Some(section), key);
                Ok(())
            }
            None => Err("Configuration not loaded".to_string()),
        }
    }
    pub fn to_string(&self) -> String {
        match &self.content {
            Some(ini) => {
                let mut result = String::new();
                for (sec, prop) in ini {
                    if sec.is_some() {
                        result.push_str(&format!("[{}]\n", sec.unwrap()));
                    }
                    for (key, value) in prop.iter() {
                        result.push_str(&format!("{}={}\n", key, value));
                    }
                }
                result
            }
            None => "".to_string(),
        }
    }
}
