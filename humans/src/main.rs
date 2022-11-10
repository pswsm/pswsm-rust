mod humans;

fn main() {
    let blood: humans::Blood = humans::Blood::from(('a', ['a', 'a']));
    let man: humans::Human = humans::Human::from((blood, "Pau", "Figueras", 18, 18));
    println!("{}", man);
}
