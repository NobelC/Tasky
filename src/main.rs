mod backend;
use backend::project::Project;

fn main() {
    let test = Project::new("Prueba", "Jeremy");
    println!("hello Tasky: {:}", test.get_id());
}
