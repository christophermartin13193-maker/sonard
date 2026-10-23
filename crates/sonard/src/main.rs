use sonar::Output;
use sonard::Automaton;

fn main() {
    let mut auto = Automaton::default();
    loop {
        match auto.run() {
            Ok(Output::Continue) => continue,
            Ok(Output::Exit(code)) => std::process::exit(code),
            Err(err) => std::process::exit(auto.handle_error(err)),
        }
    }
}
