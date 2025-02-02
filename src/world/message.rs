pub trait Message {
  fn display(&self) -> String;
}

impl Message for &str {
  fn display(&self) -> String {
    self.to_string()
  }
}

impl Message for (&str, &str) {
  fn display(&self) -> String {
    let (message, language) = self;

    match *language {
      "de" => message
        .replace("1", "eins")
        .replace("2", "zwei")
        .replace("3", "drei")
        .replace("4", "vier")
        .replace("5", "fünf")
        .replace("6", "sechs"),
      "es" => message
        .replace("1", "uno")
        .replace("2", "dos")
        .replace("3", "tres")
        .replace("4", "cuatro")
        .replace("5", "cinco")
        .replace("6", "seis"),
      "fr" => message
        .replace("1", "un")
        .replace("2", "deux")
        .replace("3", "trois")
        .replace("4", "quatre")
        .replace("5", "cinq")
        .replace("6", "six"),
      "kr" => message
        .replace("1", "일")
        .replace("2", "두")
        .replace("3", "삼")
        .replace("4", "네")
        .replace("5", "오")
        .replace("6", "육"),
      "se" => message
        .replace("1", "en")
        .replace("2", "två")
        .replace("3", "tre")
        .replace("4", "fyra")
        .replace("5", "fem")
        .replace("6", "sex"),
      "cn" => message
        .replace("1", "一")
        .replace("2", "二")
        .replace("3", "三")
        .replace("4", "四")
        .replace("5", "五")
        .replace("6", "六"),
      _ => message.to_string(),
    }
  }
}
