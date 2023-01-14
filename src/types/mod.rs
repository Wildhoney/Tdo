#[derive(Debug, PartialEq)]
pub enum Output {
    Add(Option<Task>),
    Remove(Option<Task>),
    List(Option<Vec<Task>>),
    Unactionable,
}

#[derive(Debug, PartialEq)]
pub struct Task {
    pub id: Option<usize>,
    pub description: String,
    pub completed: bool,
}
