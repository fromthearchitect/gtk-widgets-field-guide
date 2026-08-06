pub struct Task {
    pub title: String,
    pub done: bool,
}

pub fn starting_tasks() -> Vec<Task> {
    vec![
        Task {
            title: "Write the list-mindset post".to_string(),
            done: false,
        },
        Task {
            title: "Record the cover art".to_string(),
            done: false,
        },
        Task {
            title: "Reply to the GTK forum thread".to_string(),
            done: true,
        },
    ]
}
