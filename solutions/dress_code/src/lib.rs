#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black, 
    White,
    Flowers
}
#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback, 
    Baseball, 
    Fedora
}

pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
    let jacket = match formality_level {
        None => Jacket::Flowers,
        Some(0) => Jacket::Black,
        Some(_) => Jacket::White
    };

    let hat = match (formality_level, invitation_message) {
        (None, Err(_)) => Hat::Baseball,
        (_, Ok(_)) => Hat::Fedora,
        (_, _) => Hat::Snapback    
    };
    Outfit {jacket, hat}
}