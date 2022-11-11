mod humans;

fn main() {
    let man: humans::Human = humans::Human::from((humans::BLOOD_TYPES[0], "Pau", "Figueras", 18, 18));
    println!("{}", man);
}
