mod mall;
use std::collections::HashMap;
pub use mall::*;

fn all_stores(mall: &Mall) -> impl Iterator<Item = (&String, &Store)> {
    mall.floors.values().flat_map(|f| f.stores.iter())
}

fn all_employees( mall: &Mall) -> impl Iterator<Item = (&String, &Employee)> {
    all_stores(mall).flat_map(|(_, s)| s.employees.iter())
}

pub fn biggest_store(mall: &Mall) -> (String, Store) {
    all_stores(mall)
        .max_by_key(|(_, s)| s.square_meters)
        .map(|(n, s)| (n.clone(), s.clone()))
        .expect("mall has at least one store")
}

pub fn highest_paid_employee(mall: &Mall) -> Vec<(&str, Employee)> {
    let top = all_employees(mall)
        .map(|(_, e)| e.salary)
        .fold(0.0, f64::max);

    all_employees(mall)
        .filter(|(_, e)| (e.salary - top).abs() < f64::EPSILON)
        .map(|(name, e)| (name.as_str(), *e))
        .collect()
}

pub fn nbr_of_employees(mall: &Mall) -> usize {
    mall.guards.len() + all_employees(mall).count()
}

pub fn check_for_securities(mall: &mut Mall, mut pool: HashMap<String, Guard>,) {
    let total_capacity: u64 = mall.floors.values().map(|f| f.size_limit).sum();
    let required = ((total_capacity + 199) / 200) as usize; // ceiling division

    for (name, guard) in pool.drain() {
        if mall.guards.len() >= required {
            break;
        }
        mall.hire_guard(name, guard);
    }
}

pub fn cut_or_raise(mall: &mut Mall) {
    for floor in mall.floors.values_mut() {
        for store in floor.stores.values_mut() {
            for emp in store.employees.values_mut() {
                let hours = emp.working_hours.1 - emp.working_hours.0;
                let delta = emp.salary * 0.10;
                if hours >= 10 {
                    emp.raise(delta);
                } else {
                    emp.cut(delta);
                }
            }
        }
    }
}
