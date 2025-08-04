pub struct Film {
    pub name: String,
}

pub fn read_film_name(film: &Film) -> String {
    film.name.clone()
}

pub fn take_film_name(film: Film) -> String {
    film.name
}