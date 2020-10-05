use std::fs;
use xdg::BaseDirectories;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Default)]
pub struct Config {
  pub device: String,
  pub threshold: f64,
  pub left_swipe_action: String,
  pub right_swipe_action: String,
}

impl Config {
  pub fn construct(&self) -> Config {
    let yml_docs = self.get_conf_yaml();
    let conf_yaml = &yml_docs[0];

    Config {
      device: self.get_device_setting(conf_yaml),
      threshold: self.get_swipe_vdelta_threshold(conf_yaml),
      left_swipe_action: self.get_swipe_action(conf_yaml, "left"),
      right_swipe_action: self.get_swipe_action(conf_yaml, "right"),
    }
  }

  fn get_conf_filename(&self) -> String {
    let xdg_dirs = BaseDirectories::with_prefix("libinput-gestures-macos").unwrap();
    let config_path = xdg_dirs
      .place_config_file("config.ini")
      .expect("Cannot create configuration directory");

    return config_path.to_str().unwrap().to_string();
  }

  fn get_conf_yaml(&self) -> Vec<Yaml> {
    let conf_filename = self.get_conf_filename();
    let content = fs::read_to_string(conf_filename.as_str()).expect("Unable to read config file");

    return YamlLoader::load_from_str(content.as_str()).expect("Unexpected config content");
  }

  fn get_device_setting(&self, conf_yaml: &Yaml) -> String {
    return conf_yaml["device"]
      .as_str()
      .unwrap_or("/dev/input/by-path/pci-0000:00:15.1-platform-i2c_designware.1-event-mouse")
      .to_string();
  }

  fn get_swipe_vdelta_threshold(&self, conf_yaml: &Yaml) -> f64 {
    return conf_yaml["swipe"]["threshold"]["vdelta"]
      .as_f64()
      .unwrap_or(0.00175);
  }

  // fn capitalize(&self, s: &str) -> String {
  //     let mut c = s.chars();

  //     match c.next() {
  //         None => String::new(),
  //         Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
  //     }
  // }

  fn get_swipe_action(&self, conf_yaml: &Yaml, dir: &str) -> String {
    return conf_yaml["swipe"]["action"][dir]
      .as_str()
      .unwrap_or(["super+shift+", dir].concat().as_str())
      .to_string();
  }

  // pub fn get_cmd(&self, cmd: &str) -> (&str, &[&str]) {
  //     return ("aa", &["bb"]);
  // }
}
