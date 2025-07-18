use std::env;
use emojis::Emoji;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: emoji-info <emoji_or_codepoint>");
        return;
    }
    let input = &args[1];

    // Convert input like "U+1FAE9" to char or treat as raw emoji glyph
    let emoji_opt: Option<&Emoji> = if let Some(hex) = input.strip_prefix("U+") {
        if let Ok(cp) = u32::from_str_radix(hex, 16) {
            if let Some(c) = std::char::from_u32(cp) {
                emojis::get(&c.to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        emojis::get(input)
    };

    match emoji_opt {
        Some(emoji) => {
            println!("Emoji: {}", emoji.as_str());
            println!("Name: {}", emoji.name());
            println!("Shortcodes: {:?}", emoji.shortcodes().collect::<Vec<_>>());
            println!("Unicode Version: {:?}", emoji.unicode_version());  // use :? here
            println!("Group: {:?}", emoji.group());
        }
        None => eprintln!("Emoji not found or invalid input."),
    }

}