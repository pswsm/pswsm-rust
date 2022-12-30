mod humans;

fn main() {
    let bt1: humans::BloodType = humans::BloodType::A;
    let bt2: humans::BloodType = humans::BloodType::AB;
    let mani: humans::Human = humans::Human::new(bt1, "Pau", "Figueras", "2003-20-10");
    let mano: humans::Human = humans::Human::new(bt2, "Clara", "", "2003-20-10");
    let child: humans::Human = humans::Human::breed(mani, mano);
    println!("{}", child);
}
