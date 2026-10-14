pub struct Task {
    pub title: String,
    pub done: bool,
}

/// Ten tasks, so select_all() on the Multi page logs the same `(0, 10)` range
/// the post shows.
pub fn starting_tasks() -> Vec<Task> {
    vec![
        Task {
            title: "Write the selection-model post".to_string(),
            done: true,
        },
        Task {
            title: "Swap NoSelection for SingleSelection".to_string(),
            done: true,
        },
        Task {
            title: "Work out why row 0 is already selected".to_string(),
            done: true,
        },
        Task {
            title: "Move autoselect above model in the builder".to_string(),
            done: true,
        },
        Task {
            title: "Turn can-unselect on".to_string(),
            done: true,
        },
        Task {
            title: "Stop trusting selection-changed's arguments".to_string(),
            done: false,
        },
        Task {
            title: "Re-query the bitset instead".to_string(),
            done: false,
        },
        Task {
            title: "Check select_range's return value".to_string(),
            done: false,
        },
        Task {
            title: "Draft the filter and sort post".to_string(),
            done: false,
        },
        Task {
            title: "Push the companion code".to_string(),
            done: false,
        },
    ]
}
