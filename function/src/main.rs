#[derive(PartialEq)]
enum Lang {
  English,
  Spanish,
  Chinese,
  Texan,
  German,
  Italian, 
  French,
  Japanese
}

struct Greeting {
    message: String,
    lang: Lang,
}

fn main() -> Result<(), String> {
  println!("{}", greet(Lang::English)?);
  println!("{}", greet(Lang::Texan)?);
  println!("{}", greet(Lang::Spanish)?);
  println!("{}", greet(Lang::Chinese)?);
  println!("{}", greet(Lang::French)?); 
  println!("{}", greet(Lang::German)?); 
  println!("{}", greet(Lang::Japanese)?); 
  Ok(())
}

fn greet (lang : Lang) -> Result<String, String> {
  let mut v :Vec<Greeting> = Vec::new();

  let g : Greeting = Greeting { lang: Lang::English, message: String::from("Hello WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Spanish, message: String::from("Hola WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Texan, message: String::from("Howdy WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Chinese, message: String::from("WasmEdge 你好!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::German, message: String::from("guten tag WasmEdge") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Italian, message: String::from("Salve WasmEdge!") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::French, message: String::from("Bonjour WasmEdge") };
  v.push(g);
  let g : Greeting = Greeting { lang: Lang::Japanese, message: String::from("こんにちは  WasmEdge!") };
  v.push(g);

  for e in v {
    if lang == e.lang {
      return Ok(e.message);
    }
  }
  return Err("Language not found!".to_string());
}
