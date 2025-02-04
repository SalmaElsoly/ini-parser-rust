use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    fs::File,
    io::{Error, ErrorKind, Result, Write},
};

pub struct IniParser {
    sections: HashMap<String, HashMap<String, String>>,
}

impl IniParser {
    pub fn new_ini_parser() -> Self {
        IniParser {
            sections: HashMap::new(),
        }
    }

    fn set(&mut self, section: &str, key: &str, value: &str) -> Result<()> {
        let section = section.to_lowercase();
        let key = key.to_lowercase();
        let value = value.to_lowercase();
        if let None = self.sections.get(&section) {
            self.sections.insert(section.clone(), HashMap::new());
        }
        self.sections
            .get_mut(&section)
            .unwrap()
            .insert(key.clone(), value);
        if self.sections.get(&section).unwrap().get(&key).is_none() {
            return Err(Error::new(ErrorKind::Other, "failed to set value"));
        }
        Ok(())
    }

    pub fn get(&self, section: &str, key: &str) -> Option<String> {
        let section = section.to_lowercase();
        let key = key.to_lowercase();
        if let None = self.sections.get(&section) {
            return None;
        }
        self.sections.get(&section).unwrap().get(&key).cloned()
    }

    pub fn get_section_names(&self) -> Vec<String> {
        let mut sections = self.sections.keys().cloned().collect::<Vec<String>>();
        sections.sort();
        sections
    }

    pub fn get_sections(&self) -> Option<HashMap<String, HashMap<String, String>>> {
        Some(self.sections.clone())
    }

    pub fn load_from_string(&mut self, content: &str) -> Result<()> {
        let mut current_section = "".to_string();
        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("[") && line.ends_with("]") {
                current_section = line[1..line.len() - 1].to_string();
            } else if line.contains("=") && !line.starts_with("#") && !line.starts_with(";") {
                let parts: Vec<&str> = line.split("=").collect();
                if parts.len() != 2 {
                    return Err(Error::new(ErrorKind::Other, "invalid line"));
                }
                let key = parts[0].trim();
                let value = parts[1].trim();
                match self.set(&current_section, key, value) {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(())
    }

    pub fn load_from_file(&mut self, file_path: &str) -> Result<()> {
        let content = std::fs::read_to_string(file_path).unwrap();
        self.load_from_string(&content)
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<()> {
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let content = self.save_to_string();
        match file.write(content.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn save_to_string(&self) -> String {
        let mut content = String::new();
        for (section, values) in &self.sections {
            content.push_str(&format!("[{}]\n", section));
            for (key, value) in values {
                content.push_str(&format!("{}={}\n", key, value));
            }
        }
        content
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    lazy_static! {
        static ref EXPECTED_SECTIONS: HashMap<String, HashMap<String, String>> = HashMap::from([
            (
                "default".to_string(),
                HashMap::from([
                    ("forwardx11".to_string(), "yes".to_string()),
                    ("serveraliveinterval".to_string(), "45".to_string()),
                    ("compression".to_string(), "yes".to_string()),
                    ("compressionlevel".to_string(), "9".to_string()),
                ]),
            ),
            (
                "forge.example".to_string(),
                HashMap::from([("user".to_string(), "hg".to_string()),]),
            ),
            (
                "topsecret.server.example".to_string(),
                HashMap::from([
                    ("port".to_string(), "50022".to_string()),
                    ("forwardx11".to_string(), "no".to_string()),
                ]),
            ),
        ]);
    }
    #[test]
    fn test_new_ini_parser() {
        let ini_parser = IniParser::new_ini_parser();
        assert_eq!(ini_parser.sections.len(), 0);
    }

    #[test]
    fn test_load_from_file() {
        let mut ini_parser = IniParser::new_ini_parser();
        let result = ini_parser.load_from_file("testdata/config.ini");
        assert_eq!(result.is_ok(), true);
        assert_eq!(ini_parser.sections, EXPECTED_SECTIONS.clone());
    }

    #[test]
    fn test_get() {
        let mut ini_parser = IniParser::new_ini_parser();
        ini_parser.sections = EXPECTED_SECTIONS.clone();
        let get_result = ini_parser.get("default", "forwardx11");
        assert_eq!(get_result, Some(String::from("yes")));
    }

    #[test]
    fn test_get_section_names() {
        let mut ini_parser = IniParser::new_ini_parser();
        ini_parser.sections = EXPECTED_SECTIONS.clone();
        let result = ini_parser.get_section_names();
        assert_eq!(
            result,
            vec![
                "default".to_string(),
                "forge.example".to_string(),
                "topsecret.server.example".to_string()
            ]
        )
    }

    #[test]
    fn test_get_sections() {
        let mut ini_parser = IniParser::new_ini_parser();
        ini_parser.sections = EXPECTED_SECTIONS.clone();
        let result = ini_parser.get_sections();
        assert_eq!(result, Some(EXPECTED_SECTIONS.clone()))
    }

    #[test]
    fn test_load_from_string() {
        let mut ini_parser = IniParser::new_ini_parser();
        let result = ini_parser.load_from_string(
            r#"[default]
forwardx11=yes
serveraliveinterval=45
compression=yes
compressionlevel=9

[forge.example]
user=hg

[topsecret.server.example]
port=50022
forwardx11=no
"#,
        );
        assert!(result.is_ok());
        assert_eq!(ini_parser.sections, EXPECTED_SECTIONS.clone());
    }
}
