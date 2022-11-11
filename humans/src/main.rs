mod humans;

fn main() {
    let man: humans::Human = humans::Human::from((humans::BLOOD_TYPES[0], "Pau", "Figueras", 18, 18));
    println!("{}", man);
    let man2: humans::Human = humans::Human::from((humans::BLOOD_TYPES[1], "Pau", "Figueras", 18, 18));
    println!("{}", man2);
}
