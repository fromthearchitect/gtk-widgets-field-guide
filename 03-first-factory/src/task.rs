pub struct Task {
    pub title: String,
    pub done: bool,
}

pub fn starting_tasks() -> Vec<Task> {
    vec![
        Task {
            title: "Write the first-factory post".to_string(),
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
        Task {
            title: "Fix the dim-label leak".to_string(),
            done: true,
        },
        Task {
            title: "Wire up unbind".to_string(),
            done: true,
        },
        Task {
            title: "Draft the selection-model post".to_string(),
            done: false,
        },
        Task {
            title: "Update the companion repo README".to_string(),
            done: false,
        },
        Task {
            title: "Scroll far enough to trigger recycling".to_string(),
            done: true,
        },
    ]
}
