mod humans;
use humans::human;

fn main() {
    let blood: human::Blood = human::Blood::from(('a', ['a', 'a']));
    let man: human::Human = human::Human::from((blood, "Pau", "Figueras", 18, 18));
    println!("{}", man);
}
