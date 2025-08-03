use json::{ object, JsonValue };

pub struct Food {
    pub name: String,
    pub calories: (String, String), // ("2133.84kJ", "510kcal")
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

fn rond(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

pub fn calculate_macros(foods: &[Food]) -> JsonValue {
    let (mut cals, mut fats, mut carbs, mut prots) = (0.0, 0.0, 0.0, 0.0);

    for f in foods {
        let kcal = f.calories.1
            .trim_end_matches(|c: char| !c.is_ascii_digit() && c != '.')
            .parse::<f64>()
            .unwrap();

        let p = f.nbr_of_portions;
        cals += kcal * p;
        fats += f.fats * p;
        carbs += f.carbs * p;
        prots += f.proteins * p;
    }

    object! {
        "cals"     => rond(cals),
        "carbs"    => rond(carbs),
        "proteins" => rond(prots),
        "fats"     => rond(fats),
    }
}
