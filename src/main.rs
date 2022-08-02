mod cli;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() {
    if let Err(e) = cli::get_args().and_then(cli::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

// fn main() -> crossterm::Result<()> {
//     enable_raw_mode()?;

//     loop {
//         match event::read()? {
//             Event::Key(event) => {
//                 println!("Got a Keyevent: {:?}\r", event);
//                 if event == (KeyCode::Esc.into()) {
//                     break;
//                 }
//             }
//             Event::Mouse(event) => {
//                 println!("Got a Keyevent: {:?}\r", event);
//             }
//             Event::Resize(num1, num2) => {
//                 println!("Window has been resized to {:?} {}", num1, num2);
//             }
//         }
//     }

//     disable_raw_mode()
// }
