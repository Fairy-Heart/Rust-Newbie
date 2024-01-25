struct Person {
    name: String,
    age: u8,
    job: String,
}

fn create_struct() -> Box<Person> {
    let name = String::from("Kaxtr");
    let age = 19;
    let job = String::from("none");
    Box::new(Person {
            name,
            age,
            job,
        }
    )
}

fn main() {
    let person = create_struct();
    println!("{} {} {} ", person.name, person.age, person.job);
}