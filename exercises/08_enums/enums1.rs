#[derive(Debug)]
struct ResizeMsg(u32, u32);

#[derive(Debug)]
struct MoveMsg(u32, u32);

#[derive(Debug)]
struct EchoMsg(String);

#[derive(Debug)]
struct ChangeColorMsg(u8, u8, u8);

#[derive(Debug)]
struct QuitMsg();

#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Resize(ResizeMsg),
    Move(MoveMsg),
    Echo(EchoMsg),
    ChangeColor(ChangeColorMsg),
    Quit(QuitMsg),
}

fn main() {
    println!("{:?}", Message::Resize(ResizeMsg(1, 2)));
    println!("{:?}", Message::Move(MoveMsg(3, 4)));
    println!("{:?}", Message::Echo(EchoMsg("Hello".to_string())));
    println!("{:?}", Message::ChangeColor(ChangeColorMsg(7, 8, 0)));
    println!("{:?}", Message::Quit(QuitMsg()));
}
