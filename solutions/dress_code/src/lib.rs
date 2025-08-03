#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

pub fn choose_outfit(
    formality_level: Option<u32>,
    invitation_message: Result<&str, &str>
) -> Outfit {
    use Hat::*;
    use Jacket::*;

    let jacket = match formality_level {
        None => Flowers,
        Some(0) => Black,
        Some(_) => White,
    };

    let hat = match (formality_level, invitation_message) {
        (None, Err(_)) => Baseball,
        (_, Ok(_)) => Fedora,
        _ => Snapback,
    };

    Outfit { jacket, hat }
}
