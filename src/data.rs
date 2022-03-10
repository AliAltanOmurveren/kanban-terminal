use std::time::Duration;
use std::collections::BTreeMap;

use tui::widgets::ListState;

use serde_with::{serde_as, DurationSeconds};
use serde::{Deserialize, Serialize};

// * State of the App
pub struct App {
    pub tab: BTreeMap<i32, String>,
    pub test_int: i32,
    pub daily_task: DailyTask,
    pub kanban: Kanban,
    pub focus: Focus,
    pub chunk_size: Vec<i32>,
    pub messages: Message,
    pub popup: Popup,
    pub input: String,
    pub can_input: bool
}

impl App {

    pub fn default() -> App {

        App{
            tab: BTreeMap::from([
                (3, format!("{:^1$}", "Kanban", "Kanban".len() + 2)),
            ]),
            test_int: 5,
            daily_task: DailyTask::default(),
            kanban: Kanban::default(),
            focus: Focus::default(),
            chunk_size: vec![0, 1, 0, 2], // actual size -1 for indexing
            messages: Message::default(),
            popup: Popup::Disabled,
            input: String::from(""),
            can_input: false
        }
    }
}

pub enum Popup {
    AddTask,
    EditTask,
    DeleteTask,
    AddProject,
    EditProject,
    DeleteProject,
    AddTodo,
    EditTodo,
    DeleteTodo,
    AddInProgress,
    EditInProgress,
    DeleteInProgress,
    AddDone,
    EditDone,
    DeleteDone,
    Disabled
}

pub struct Message {
    pub quit: String,
    pub tab_change: String
}

impl Message {

    pub fn default() -> Message {

        Message {
            quit: String::from("ESC to exit"),
            tab_change: String::from("Ctrl + arrows ot change tabs")
        }
    }
}

pub struct Focus{
    pub tab_focus: i32,
    pub chunk_focus: Vec<i32>
}

impl Focus{

    pub fn default() -> Focus {

        Focus {
            tab_focus: 3,
            chunk_focus: vec![0,0,0,0]
        }
    }
}

pub struct DailyTask {
    pub tasks: Vec<Task>,
    pub selected_task_index: usize,
    pub selected_step_index: usize,
    pub daily_task_list_state: ListState,
    pub daily_task_step_list_state: ListState
}

impl DailyTask {

    pub fn default() -> DailyTask {

        DailyTask{
            tasks: Vec::new(),
            selected_task_index: 0,
            selected_step_index: 1000,
            daily_task_list_state: ListState::default(),
            daily_task_step_list_state: ListState::default(),
        }
    }

    pub fn add_task(&mut self, task: Task) {

        self.tasks.push(task);
    }
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct Task {
    pub task_name: String,
    pub steps: Vec<TaskStep>,
    #[serde_as(as = "DurationSeconds<u64>")]
    pub task_duration: Duration
}

impl Task {

    pub fn new(name: &str) -> Task {

        Task{
            task_name: String::from(name),
            steps: Vec::new(),
            task_duration: Duration::from_secs(0)
        }
    }

    pub fn add_step(&mut self, step: TaskStep) {

        self.steps.push(step);
    }
}

#[serde_as]
#[derive(Deserialize, Serialize)]
pub struct TaskStep {
    pub step_name: String,
    #[serde_as(as = "DurationSeconds<u64>")]
    pub step_duration: Duration
}

impl TaskStep {

    pub fn new(name: &str, step_duration: Duration) -> TaskStep {

        TaskStep {
            step_name: String::from(name),
            step_duration
        }
    }

    // ! Ignored 
    pub fn _update_step(&mut self, step_name: String, step_duration: Duration) {

        self.step_name = step_name;
        self.step_duration = step_duration;
    }
}

pub struct Kanban {
    pub projects : Vec<KanbanProject>,
    pub project_index : usize,
    pub todo_state : ListState,
    pub todo_index : usize,
    pub in_progress_state : ListState,
    pub in_progress_index : usize,
    pub done_state : ListState,
    pub done_index : usize
}

impl Kanban {

    pub fn default() -> Kanban {
        
        Kanban {
            projects: Vec::new(),
            project_index: 0,
            todo_state: ListState::default(),
            todo_index: 0,
            in_progress_state: ListState::default(),
            in_progress_index: 1000,
            done_state: ListState::default(),
            done_index: 1000
        }
    }

    pub fn add_project(&mut self, project: KanbanProject) {

        self.projects.push(project);
    }

    pub fn add_todo(&mut self, index: usize, todo_name: &str) {

        self.projects[index].todo.push(String::from(todo_name));
    }

    pub fn add_in_progress(&mut self, index: usize, in_progress_name: &str) {

        self.projects[index].in_progress.push(String::from(in_progress_name));
    }

    pub fn add_done(&mut self, index: usize, done_name: &str) {

        self.projects[index].done.push(String::from(done_name));
    }
}

#[derive(Serialize, Deserialize)]
pub struct KanbanProject {
    pub name: String,
    pub todo : Vec<String>,
    pub in_progress :  Vec<String>,
    pub done : Vec<String>
}

impl KanbanProject {

    pub fn new(name: String) -> KanbanProject {
        KanbanProject {
            name,
            todo: Vec::new(),
            in_progress: Vec::new(),
            done: Vec::new()
        }
    }
}
