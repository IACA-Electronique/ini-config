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

    pub fn get(&self, section: &str, key: &str) -> Vec<String> {
        let mut result = Vec::new();
        if let Some(ref ini) = self.content {
            if section == "" {
                let value = ini.general_section().get_all(key);
                let collected: Vec<String> = value.map(String::from).collect();
                result.extend(collected);
            } else {
                if let Some(s) = ini.section(Some(section)) {
                    let value = s.get_all(key);
                    let collected: Vec<String> = value.map(String::from).collect();
                    result.extend(collected);
                }
            }
        }
        result
    }
    
    pub fn exists(&self, section: &str, key: &str) -> bool {
        !self.get(section, key).is_empty()
    }
    
    pub fn set(&mut self, section: &str, key: &str, value: &str) -> Result<(), String> {
        match &mut self.content {
            Some(ini) => {
                if section == "" {
                    ini.with_general_section().set(key, value);
                } else {
                    ini.with_section(Some(section)).set(key, value);
                }
                Ok(())
            }
            None => Err("Configuration not loaded".to_string()),
        }
    }
    
    pub fn add(&mut self, section: &str, key: &str, value: &str) -> Result<(), String> {
        if !self.content.is_none() {
            let existing_value = self.get(section, key);
            if existing_value.contains(&value.to_string()) {
                return Ok(());
            }

            let ini = self.content.as_mut().unwrap();
            if section == "" {
                ini.with_general_section().add(key, value);
            } else {
                ini.with_section(Some(section)).add(key, value);
            }

            Ok(())
        } else {
            Err("Configuration not loaded".to_string())
        }
    }

    pub fn delete(&mut self, section: &str, key: &str) -> Result<(), String> {
        match &mut self.content {
            Some(ini) => {
                if section == "" {
                    ini.delete_from::<&str>(None, key);
                }else {
                    ini.delete_from(Some(section), key);
                }
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
