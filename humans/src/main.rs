mod humans;

fn main() {
    let man: humans::Human = humans::Human::from((humans::BLOOD_TYPES[0], "Pau", "Figueras", 18, 18));
    let man2: humans::Human = humans::Human::from((humans::BLOOD_TYPES[1], "Paula", "Tres", 18, 18));
    let child: humans::Human = humans::Human::breed(man, man2);
    println!("{}", child);
}
