// * Std modules
use std::{
    io,

    time::Duration,

    fs,

    path
};

// * Crossterm Modules
use crossterm::{
    self, execute,

    event::{DisableMouseCapture, EnableMouseCapture, 
        poll, read, Event, KeyCode, KeyModifiers, KeyEvent},

    terminal::{disable_raw_mode, enable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen}
};

// * Tui-rs Modules
use tui::{
    Terminal, Frame,

    backend::{Backend, CrosstermBackend},

    widgets::{Block, Borders, BorderType
        , Tabs, List, ListItem, Paragraph
        , Clear, Table, Row, Cell},

    layout::{Layout, Constraint, Direction, Alignment, Rect},

    style::{Color, Style, Modifier},

    text::{Span, Spans}
};

use chrono::prelude::*;

use serde_json::Result as seresult;

mod data;
use crate::data::*;

fn main() -> Result<(), io::Error> {

    // * Terminal Setup 
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    // Run the app in new terminal screen
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    
    // Backend setup
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize app data
    let mut app: App = App::default();
    deserialize_kanban(&mut app)?;

    // Enter main loop function
    run_app(&mut terminal, &mut app)?;

    // End of the execution
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;

    disable_raw_mode()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {

    // * Main Loop
    loop {
        terminal.draw(|f| ui(f, app))?;

        if poll(Duration::from_millis(100))?{
            match read()?{
                Event::Key(key) => {
                    
                    // * Special keys

                    match key.code {
                        KeyCode::Esc => {
                            // Handle exit sequence and closing the popups
                            if let Popup::Disabled = app.popup {
                                break
                            } else {
                                close_popup(app);
                            }
                        },

                        KeyCode::Backspace => {

                            if app.input.len() > 0 && app.can_input{
                                app.input.pop();
                            }
                        },

                        KeyCode::Delete => {

                            if key.modifiers == KeyModifiers::CONTROL {
                                // delete kanban project
                                if app.kanban.projects.len() > 0{
                                    open_delete_popup(app, Popup::DeleteProject, app.kanban.projects[app.kanban.project_index].name.clone());
                                }
                            } else {

                                if app.focus.tab_focus == 1 {
                                    // daily tasks
                    
                                }else if app.focus.tab_focus == 2 {
                                    // events
                    
                                }else if app.focus.tab_focus == 3 {
                                    // kanban

                                    // "projects" is not empty
                                    if app.kanban.projects.len() > 0 {

                                        if app.focus.chunk_focus[3] == 0 {
                                            // to do
                                            if app.kanban.projects[app.kanban.project_index].todo.len() > 0 {
                                                open_delete_popup(app, Popup::DeleteTodo, 
                                                    app.kanban.projects[app.kanban.project_index].todo[app.kanban.todo_index].clone());
                                            }
                                            
                                        }else if app.focus.chunk_focus[3] == 1 {
                                            // in progress
        
                                            if app.kanban.projects[app.kanban.project_index].in_progress.len() > 0 {
                                                open_delete_popup(app, Popup::DeleteInProgress, 
                                                    app.kanban.projects[app.kanban.project_index].in_progress[app.kanban.in_progress_index].clone());
                                            }
                                            
                                        }else if app.focus.chunk_focus[3] == 2 {
                                            // done
                                            
                                            if  app.kanban.projects[app.kanban.project_index].done.len() > 0 {
                                                open_delete_popup(app, Popup::DeleteDone, 
                                                    app.kanban.projects[app.kanban.project_index].done[app.kanban.done_index].clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        KeyCode::Enter => {

                            if key.modifiers == KeyModifiers::CONTROL {
                                if app.can_input {
                                    app.input.push('\n');
                                }
                            }else{
                                match app.popup {
                                    Popup::AddProject => {
                                        if ! app.input.is_empty() {
                                            // input is not empty -> add a new kanban project
                                            app.kanban.add_project(KanbanProject::new(String::from(&app.input[..])));
        
                                            // close the popup
                                            close_popup(app);
        
                                            // update the json file for kanban
                                            serialize_kanban(app);
        
                                        }else {
                                            // input is empty -> close the popup
                                            close_popup(app);
                                        }
                                    },
    
                                    Popup::AddTodo => {
                                        if ! app.input.is_empty() {
                                            app.kanban.add_todo(app.kanban.project_index, &app.input);
        
                                            // close the popup
                                            close_popup(app);
        
                                            // update the json file for kanban
                                            serialize_kanban(app);
                                        }else {
                                            // input is empty -> close the popup
                                            close_popup(app);
                                        }
                                    },
    
                                    Popup::AddInProgress => {
                                        if ! app.input.is_empty() {
                                            app.kanban.add_in_progress(app.kanban.project_index, &app.input);
        
                                            // close the popup
                                            close_popup(app);
        
                                            // update the json file for kanban
                                            serialize_kanban(app);
                                        }else {
                                            // input is empty -> close the popup
                                            close_popup(app);
                                        }
                                    },
    
                                    Popup::AddDone => {
                                        if ! app.input.is_empty() {
                                            app.kanban.add_done(app.kanban.project_index, &app.input);
        
                                            // close the popup
                                            close_popup(app);
        
                                            // update the json file for kanban
                                            serialize_kanban(app);
                                        }else {
                                            // input is empty -> close the popup
                                            close_popup(app);
                                        }
                                    },
    
                                    Popup::EditProject => {
                                        if ! app.input.is_empty() {
                                            app.kanban.projects[app.kanban.project_index].name = app.input.to_string();
    
                                            close_popup(app);
    
                                            serialize_kanban(app);
                                        } else {
                                            // input is empty -> old name
                                            close_popup(app);
                                        }
                                    },
    
                                    Popup::EditTodo => {
                                        if ! app.input.is_empty() {
                                            app.kanban.projects[app.kanban.project_index]
                                                .todo[app.kanban.todo_index] = app.input.to_string();
    
                                            close_popup(app);
    
                                            serialize_kanban(app);
                                        } else {
                                            // input is empty -> old name
                                            close_popup(app);
                                        }
                                    },

                                    Popup::EditInProgress => {
                                        if ! app.input.is_empty() {
                                            app.kanban.projects[app.kanban.project_index]
                                                .in_progress[app.kanban.in_progress_index] = app.input.to_string();
    
                                            close_popup(app);
    
                                            serialize_kanban(app);
                                        } else {
                                            // input is empty -> old name
                                            close_popup(app);
                                        }
                                    },

                                    Popup::EditDone => {
                                        if ! app.input.is_empty() {
                                            app.kanban.projects[app.kanban.project_index]
                                                .done[app.kanban.done_index] = app.input.to_string();
    
                                            close_popup(app);
    
                                            serialize_kanban(app);
                                        } else {
                                            // input is empty -> old name
                                            close_popup(app);
                                        }
                                    },

                                    Popup::DeleteTodo => {
                                        delete_todo(app);
                                        close_popup(app);
                                        serialize_kanban(app);
                                    },

                                    Popup::DeleteInProgress => {
                                        delete_in_progress(app);
                                        close_popup(app);
                                        serialize_kanban(app);
                                    },

                                    Popup::DeleteDone => {
                                        delete_done(app);
                                        close_popup(app);
                                        serialize_kanban(app);
                                    },

                                    Popup::DeleteProject => {
                                        if app.kanban.project_index == app.kanban.projects.len() - 1 {

                                            if app.kanban.project_index == 0 {
                                                app.kanban.projects.remove(app.kanban.project_index);
                                            }else {
                                                app.kanban.projects.remove(app.kanban.project_index);
                                                app.kanban.project_index = app.kanban.projects.len() - 1;
                                            }
                                        }else {
                                            app.kanban.projects.remove(app.kanban.project_index);
                                        }

                                        close_popup(app);
                                        serialize_kanban(app);
                                    },
    
                                    _ => ()
                                }
                            }
                        }

                        _ => handle_keyboard(app, key),
                    }
                },
                Event::Mouse(_mouse) => {
                    //println!("{:?}", mouse);
                },
                Event::Resize(_width, _height) => {
                    //println!("{}, {}", width, height);
                }
            }
        }// Else: timeout
        
    }

    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    kanban_ui(f, app)
}



fn kanban_ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // * Chunk Initialization
    let top_chunks = kanban_top_chunks_init(f);

    // App windows bar init and render
    let tabs = tab_bar(app);
    f.render_widget(tabs, top_chunks[0]);

    // Date bar init and render
    let date_bar = date_bar();
    for bar in date_bar{
        f.render_widget(bar, top_chunks[1]);
    }

    let body_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(1,3),
                    Constraint::Ratio(1,3),
                    Constraint::Ratio(1,3),
                ].as_ref()
            ).split(top_chunks[3]);
    
    // There is at least one project
    if app.kanban.projects.len() > 0{
        // Project name
        let project_name = Paragraph::new(Span::from(&app.kanban.projects[app.kanban.project_index].name[..]))
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().bg(Color::Cyan).fg(Color::Black).add_modifier(Modifier::BOLD));

        f.render_widget(project_name, top_chunks[2]);

        let todo = &app.kanban.projects[app.kanban.project_index].todo;
        let todo: Vec<ListItem> = strings_to_listitem_vec(&todo) ;
        let todo = List::new(todo)
                                .block(Block::default()
                                .title(Span::styled("  ToDo  ", if app.focus.chunk_focus[3] == 0 {
                                    Style::default().bg(Color::Cyan).fg(Color::Black)
                                }else {
                                    Style::default()
                                }))
                                .title_alignment(Alignment::Center)
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                            )
                            .style(Style::default().fg(Color::White))
                            .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
                            .highlight_symbol(" ❱ ");

        let in_progress = &app.kanban.projects[app.kanban.project_index].in_progress;
        let in_progress: Vec<ListItem> = strings_to_listitem_vec(&in_progress) ;
        let in_progress = List::new(in_progress)
                                .block(Block::default()
                                .title(Span::styled("  In Progress  ", if app.focus.chunk_focus[3] == 1 {
                                    Style::default().bg(Color::Cyan).fg(Color::Black)
                                }else {
                                    Style::default()
                                }))
                                .title_alignment(Alignment::Center)
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                            )
                            .style(Style::default().fg(Color::White))
                            .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
                            .highlight_symbol(" ❱ ");

        let done = &app.kanban.projects[app.kanban.project_index].done;
        let done: Vec<ListItem> = strings_to_listitem_vec(&done) ;
        let done = List::new(done)
                                .block(Block::default()
                                .title(Span::styled("  Done  ", if app.focus.chunk_focus[3] == 2 {
                                    Style::default().bg(Color::Cyan).fg(Color::Black)
                                }else {
                                    Style::default()
                                }))
                                .title_alignment(Alignment::Center)
                                .borders(Borders::ALL)
                                .border_type(BorderType::Rounded)
                            )
                            .style(Style::default().fg(Color::White))
                            .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black))
                            .highlight_symbol(" ❱ ");

        app.kanban.todo_state.select(Option::from(app.kanban.todo_index));
        app.kanban.in_progress_state.select(Option::from(app.kanban.in_progress_index));
        app.kanban.done_state.select(Option::from(app.kanban.done_index));

        f.render_stateful_widget(todo, body_chunks[0], &mut app.kanban.todo_state);
        f.render_stateful_widget(in_progress, body_chunks[1], &mut app.kanban.in_progress_state);
        f.render_stateful_widget(done, body_chunks[2], &mut app.kanban.done_state);
    }else {
        //there is not any project

        let project_name = Paragraph::new(Span::from("Create a new project!"))
        .block(Block::default())
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::RAPID_BLINK));

        f.render_widget(project_name, top_chunks[2]);
    }
    
    // Popup rendering
    match app.popup {
        Popup::AddProject => show_popup(f, app, "Add a New Project", Color::Cyan),
        Popup::AddTodo => show_popup(f, app, "Add a New Todo Task", Color::Yellow),
        Popup::AddInProgress => show_popup(f, app, "Add a new In Progress Task", Color::Yellow),
        Popup::AddDone => show_popup(f, app, "Add a New Done Task", Color::Yellow),
        Popup::EditProject => show_popup(f, app, "Edit Project Name", Color::Cyan),
        Popup::EditTodo => show_popup(f, app, "Edit Todo Task's Name", Color::Yellow),
        Popup::EditInProgress => show_popup(f, app, "Edit Todo Task's Name", Color::Yellow),
        Popup::EditDone => show_popup(f, app, "Edit Done Task's Name", Color::Yellow),
        Popup::DeleteTodo => show_popup(f, app, "Delete Todo?", Color::Red),
        Popup::DeleteInProgress => show_popup(f, app, "Delete In Progress?", Color::Red),
        Popup::DeleteDone => show_popup(f, app, "Delete Done?", Color::Red),
        Popup::DeleteProject => show_popup(f, app, "Delete Current Project?", Color::Red),
        _ => ()
    }
    
}

// * Helper functions

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}

fn tab_bar(app: &mut App) -> Tabs {
    let titles = app.tab.values().cloned().map(Spans::from).collect();
    Tabs::new(titles)
        .block(Block::default())
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Green))
        .divider("-")
        .select(0)
}

fn strings_to_listitem_vec(strings: &Vec<String>) -> Vec<ListItem> {
    let mut items: Vec<ListItem> = Vec::new(); 

    for s in strings {
        items.push(ListItem::new(&s[..]));
    }

    items
}

fn kanban_top_chunks_init<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
    Layout::default()
            .direction(Direction::Vertical)
            .horizontal_margin(1)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(3),
                    Constraint::Length(1),
                    Constraint::Min(5),
                ].as_ref()
            ).split(f.size())
}

fn date_bar<'a>() -> Vec<Paragraph<'a>>{
    vec![
        Paragraph::new(Span::raw(
            Local::now().format("    %d-%m-%Y").to_string()))
            .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
            .alignment(Alignment::Left)
            .style(Style::default().fg(Color::White)),
            
        Paragraph::new(Span::raw(
            Local::now().format("%A").to_string()))
            .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White)),

        Paragraph::new(Span::raw(
            Local::now().format("%H:%M    ").to_string()))
            .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
            .alignment(Alignment::Right)
            .style(Style::default().fg(Color::White))
    ]
}

fn handle_keyboard(app: &mut App, key: KeyEvent){
    let tab_focus = app.focus.tab_focus;
    let chunk_focus = &app.focus.chunk_focus;
    let selected_task_index = app.daily_task.selected_task_index;
    let selected_step_index = app.daily_task.selected_step_index;

    // * Arrow keys
    match key.code {

        KeyCode::Left => {

            // can use only if there is not any popups active
            if let Popup::Disabled = app.popup{
                // change tab focus
                if key.modifiers == KeyModifiers::CONTROL {

                    //app.focus.tab_focus = (tab_focus - 1) * (tab_focus - 1 > 0) as i32;

                }else if key.modifiers == KeyModifiers::SHIFT {
                    // * Kanban
                    if tab_focus == 3 && app.focus.chunk_focus[3] == 1 {

                        // * todo <- in_progress - done
                        if app.kanban.projects[app.kanban.project_index].in_progress.len() > 0{

                            app.kanban.add_todo(app.kanban.project_index, 
                                &app.kanban.projects[app.kanban.project_index].in_progress[app.kanban.in_progress_index].clone());

                            delete_in_progress(app);
                            serialize_kanban(app);
                        }
                    }else if tab_focus == 3 && app.focus.chunk_focus[3] == 2 {

                        // * todo - in_progress <- done 

                        if app.kanban.projects[app.kanban.project_index].done.len() > 0 {
                            app.kanban.add_in_progress(app.kanban.project_index, 
                                &app.kanban.projects[app.kanban.project_index].done[app.kanban.done_index].clone());

                            delete_done(app);
                            serialize_kanban(app);
                        }
                    }

                }else{

                    // * Daily Task
                    // cancel step selection for visual cue
                    if tab_focus == 1 && app.focus.chunk_focus[1] == 1 {
                        app.daily_task.daily_task_step_list_state.select(Option::from(1000));
                        app.daily_task.selected_step_index = 1000;
                    }

                    // * Kanban
                    if tab_focus == 3 && app.focus.chunk_focus[3] == 1 {

                        // * todo <- in_progress - done
                        app.kanban.todo_index = 0;
                        app.kanban.in_progress_index = 1000;

                    }else if tab_focus == 3 && app.focus.chunk_focus[3] == 2 {

                        // * todo - in_progress <- done 
                        app.kanban.in_progress_index = 0;
                        app.kanban.done_index = 1000;
                    }

                    // change chunk focus
                    app.focus.chunk_focus[tab_focus as usize] = chunk_focus[tab_focus as usize] - 1 * (chunk_focus[tab_focus as usize] > 0) as i32;

                }
            }
        },

        KeyCode::Right => {
            // can use only if there is not any popups active
            if let Popup::Disabled = app.popup{
                // change tab focus
                if key.modifiers == KeyModifiers::CONTROL {

                    //app.focus.tab_focus = if tab_focus + 1 >= app.tab.len() as i32 {(app.tab.len() - 1) as i32} else {tab_focus + 1};

                } else if key.modifiers == KeyModifiers::SHIFT {
                    // * Kanban
                    if tab_focus == 3 && app.focus.chunk_focus[3] == 0 {
                        // * todo -> in_progress - done
                        if app.kanban.projects[app.kanban.project_index].todo.len() > 0 {
                            app.kanban.add_in_progress(app.kanban.project_index, 
                                &app.kanban.projects[app.kanban.project_index].todo[app.kanban.todo_index].clone());
                                
                            delete_todo(app);
                            serialize_kanban(app);
                        }
                    }else if tab_focus == 3 && app.focus.chunk_focus[3] == 1 {
                        // * todo - in_progress -> done 

                        if app.kanban.projects[app.kanban.project_index].in_progress.len() > 0 {
                            app.kanban.add_done(app.kanban.project_index, 
                                &app.kanban.projects[app.kanban.project_index].in_progress[app.kanban.in_progress_index].clone());
                            
                            delete_in_progress(app);
                            serialize_kanban(app);
                        }
                    }
                } else { 

                    // * Daily Task
                    // step selection for visual cue
                    if tab_focus == 1 && app.focus.chunk_focus[1] == 0 {

                        app.daily_task.daily_task_step_list_state.select(Option::from(0));
                        app.daily_task.selected_step_index = 0;
                    }

                    // * Kanban
                    if tab_focus == 3 && app.focus.chunk_focus[3] == 0 {

                        // * todo -> in_progress - done
                        app.kanban.todo_index = 1000;
                        app.kanban.in_progress_index = 0;

                    }else if tab_focus == 3 && app.focus.chunk_focus[3] == 1 {
                        // * todo - in_progress -> done 
                        app.kanban.in_progress_index = 1000;
                        app.kanban.done_index = 0;
                    }

                    // change chunk focus
                    app.focus.chunk_focus[tab_focus as usize] = 
                        if app.focus.chunk_focus[tab_focus as usize] + 1 >= app.chunk_size[tab_focus as usize] {app.chunk_size[tab_focus as usize]}
                            else {chunk_focus[tab_focus as usize] + 1};
                    
                }
            }
        },

        KeyCode::Up => {
            // can use only if there is not any popups active
            if let Popup::Disabled = app.popup{
                if key.modifiers == KeyModifiers::CONTROL{
                    // Kanban project up
                    if app.kanban.projects.len() > 0 && tab_focus == 3{
                        app.kanban.project_index = if app.kanban.project_index + 1 < app.kanban.projects.len(){
                                                        app.kanban.project_index + 1
                                                    }else {
                                                        app.kanban.projects.len() - 1
                                                    };
    
                        adjust_kanban_indexes_upon_project_change(app);
    
                    }
                }else{
                    // * Daily Task
                    if tab_focus == 1 && chunk_focus[1] == 0 {
    
                        // focus is on the tasks
                        app.daily_task.selected_task_index = 
                            if selected_task_index <= 0 {0} 
                                else {selected_task_index - 1};
    
                    } else if tab_focus == 1 && chunk_focus[1] == 1 {
                        
                        // focus is on the steps
                        app.daily_task.selected_step_index =
                            if selected_step_index <= 0 {0}
                                else {selected_step_index - 1};
                        
                        app.daily_task.daily_task_step_list_state.select(Option::from(selected_step_index));
                    }
    
                    // * Kanban
    
                    if tab_focus == 3 && chunk_focus[3] == 0 {
    
                        // * todo up
                        app.kanban.todo_index = if app.kanban.todo_index <= 0 {0} else {app.kanban.todo_index - 1};
    
                    }else if tab_focus == 3 && chunk_focus[3] == 1 {
    
                        // * in progress up
                        app.kanban.in_progress_index = if app.kanban.in_progress_index <= 0 {0} else {app.kanban.in_progress_index - 1};
    
                    }else if tab_focus == 3 && chunk_focus[3] == 2 {
    
                        // * done up
                        app.kanban.done_index = if app.kanban.done_index <= 0 {0} else {app.kanban.done_index - 1};
                    }
                }
            }
        },

        KeyCode::Down => {
            // can use only if there is not any popups active
            if let Popup::Disabled = app.popup{
                if key.modifiers == KeyModifiers::CONTROL{
                    // Kanban project down
                    if app.kanban.projects.len() > 0 && tab_focus == 3 {
                        app.kanban.project_index = if app.kanban.project_index as i32 - 1 <= 0 {
                                                        0
                                                    }else {
                                                        app.kanban.project_index - 1
                                                    };
    
                        adjust_kanban_indexes_upon_project_change(app);
                    }
                }else {
                    // * Daily Task
    
                    if tab_focus == 1 && chunk_focus[1] == 0 {
    
                        // focus is on the tasks
                        app.daily_task.selected_task_index = 
                            if selected_task_index + 1 >= app.daily_task.tasks.len()
                                {app.daily_task.tasks.len() - 1} 
                                    else {selected_task_index + 1};
    
                        app.daily_task.daily_task_step_list_state.select(Option::from(1000));
    
                    } else if tab_focus == 1 && chunk_focus[1] == 1 {
                        
                        // focus is on the steps
                        app.daily_task.selected_step_index =
                            if selected_step_index + 1 >= app.daily_task.tasks[selected_task_index].steps.len()
                                {app.daily_task.tasks[selected_task_index].steps.len() - 1}
                                    else {selected_step_index + 1};
    
                        app.daily_task.daily_task_step_list_state.select(Option::from(app.daily_task.selected_step_index));
                    }
    
                    // * Kanban  
    
                    if tab_focus == 3 && chunk_focus[3] == 0 {
    
                        // * todo down
                        if app.kanban.projects[app.kanban.project_index].todo.len() > 0 {
                            app.kanban.todo_index = 
                                if app.kanban.todo_index + 1 >= app.kanban.projects[app.kanban.project_index].todo.len() {
                                    app.kanban.projects[app.kanban.project_index].todo.len() - 1
                                } else {app.kanban.todo_index + 1};
                        }
                    }else if tab_focus == 3 && chunk_focus[3] == 1 {
    
                        // * in progress down
                        if app.kanban.projects[app.kanban.project_index].in_progress.len() > 0 {
                            app.kanban.in_progress_index = 
                                if app.kanban.in_progress_index + 1 >= app.kanban.projects[app.kanban.project_index].in_progress.len(){
                                    app.kanban.projects[app.kanban.project_index].in_progress.len() - 1
                                } else {app.kanban.in_progress_index + 1};
                        }
                        
    
                    }else if tab_focus == 3 && chunk_focus[3] == 2 {
    
                        // * done down
                        if app.kanban.projects[app.kanban.project_index].done.len() > 0 {
                            app.kanban.done_index = 
                                if app.kanban.done_index + 1 >= app.kanban.projects[app.kanban.project_index].done.len() {
                                    app.kanban.projects[app.kanban.project_index].done.len() - 1
                                } else {app.kanban.done_index + 1};
                        }
                        
                    }
                }
            }
        },
        _ => handle_input(app, key)
    }
                
}

// TODO here
fn handle_input(app: &mut App, key: KeyEvent) {

    // * char keys

    match key.code {
        KeyCode::Char(c) => {
            if c == 'n' {

                handle_n_key(key, app);
                
            } else if c == 'e' {

                handle_e_key(key, app);

            } else {
                if app.can_input {
                    app.input.push(c);
                }
            }
            
        },
        _ => {}
    }
}

fn handle_e_key(key: KeyEvent, app: &mut App){
    if key.modifiers == KeyModifiers::CONTROL {
        // ctrl + e is only for kanban project
        if app.focus.tab_focus == 3 {
            open_edit_popup(app, Popup::EditProject, app.kanban.projects[app.kanban.project_index].name.clone());
        }
    }else {
        if app.can_input {
            app.input.push('e');
        }else{
            // edit operations go here

            if app.focus.tab_focus == 1 {
                // daily tasks

            }else if app.focus.tab_focus == 2 {
                // events

            }else if app.focus.tab_focus == 3 {
                // kanban

                if app.focus.chunk_focus[3] == 0 {
                    // to do
                    if app.kanban.projects[app.kanban.project_index].todo.len() > 0 {
                        open_edit_popup(app, Popup::EditTodo, app.kanban.projects[app.kanban.project_index]
                                                                .todo[app.kanban.todo_index].clone());
                    }
                }else if app.focus.chunk_focus[3] == 1 {
                    // in progress
                    if app.kanban.projects[app.kanban.project_index].in_progress.len() > 0 {
                        open_edit_popup(app, Popup::EditInProgress, app.kanban.projects[app.kanban.project_index]
                            .in_progress[app.kanban.in_progress_index].clone());
                    }
                }else if app.focus.chunk_focus[3] == 2 {
                    // done
                    if app.kanban.projects[app.kanban.project_index].done.len() > 0 {
                        open_edit_popup(app, Popup::EditDone, app.kanban.projects[app.kanban.project_index]
                            .done[app.kanban.done_index].clone());
                    }
                }
            }
        }
    }
}

fn handle_n_key(key: KeyEvent, app: &mut App){
    // ctrl + n is only for kanban project
    if key.modifiers == KeyModifiers::CONTROL {
        if app.focus.tab_focus == 3{
            open_popup(app, Popup::AddProject);
        }
    } else {
        if app.can_input {
            app.input.push('n');
        }else{
            // new operations go here

            if app.focus.tab_focus == 1 {
                // daily tasks

            }else if app.focus.tab_focus == 2 {
                // events

            }else if app.focus.tab_focus == 3 {
                // kanban

                if app.focus.chunk_focus[3] == 0 {
                    // to do
                    
                    open_popup(app, Popup::AddTodo);
                    

                }else if app.focus.chunk_focus[3] == 1 {
                    // in progress

                    open_popup(app, Popup::AddInProgress);

                }else if app.focus.chunk_focus[3] == 2 {
                    // done

                    open_popup(app, Popup::AddDone);
                }
            }
        }
    }
}

fn deserialize_kanban(app: &mut App) -> seresult<()>{

    let p= std::env::current_exe().unwrap();
    let mut json_path = String::from(p.parent().unwrap().to_str().unwrap());
    json_path.push_str("\\kanban.json");

    let data = match fs::read_to_string(&json_path){
        Ok(s) => s,
        Err(_) => {
            fs::File::create(&json_path).unwrap();
            fs::read_to_string(&json_path).unwrap()
        }
    };

    let project: Vec<KanbanProject> = match serde_json::from_str(&data){ 
        Ok(s) => s,
        Err(_) => Vec::new()
    } ;

    for p in project {
        app.kanban.add_project(p);
    }

    Ok(())
}

fn serialize_kanban(app: &mut App) {

    let p= std::env::current_exe().unwrap();
    let mut json_path = String::from(p.parent().unwrap().to_str().unwrap());
    json_path.push_str("\\kanban.json");

    let j = serde_json::to_string_pretty(&app.kanban.projects).unwrap();

    fs::write(json_path, j).unwrap();
}

fn open_popup(app: &mut App, popup: Popup) {
    app.input = "".to_string();
    app.popup = popup;
    app.can_input = true;
}

fn close_popup(app: &mut App) {
    app.input = "".to_string();
    app.popup = Popup::Disabled;
    app.can_input = false;
}

fn show_popup<B: Backend>(f: &mut Frame<B>, app: &mut App, title: &str, color: Color) {
    let popup_block = Block::default()
                                    .title(format!("  {}  ", title))
                                    .title_alignment(Alignment::Center)
                                    .style(Style::default().bg(color).fg(Color::Black))
                                    .borders(Borders::ALL);

        let input_area = Paragraph::new(Span::from(format!("|{}|",&app.input[..])))
                            .block(Block::default()
                                .style(Style::default().bg(color).fg(Color::Black))
                            )
                            .alignment(Alignment::Center);

        let popup_area = centered_rect(60, 20, f.size());

        let margin_layout = Layout::default()
                                .constraints(
                                    [
                                        Constraint::Min(5)
                                    ]
                                ).split(popup_area);

        let margin_layout2 = Layout::default()
                                .vertical_margin(
                                    if margin_layout[0].height % 2 == 0 {
                                        margin_layout[0].height / 2 - 1
                                    }else {
                                        margin_layout[0].height / 2
                                    }
                                )
                                .constraints(
                                    [
                                        Constraint::Min(3)
                                    ]
                                ).split(margin_layout[0]);

        f.render_widget(Clear, popup_area);
        f.render_widget(popup_block, margin_layout[0]);
        f.render_widget(input_area, margin_layout2[0]);
}

fn open_edit_popup(app: &mut App, popup: Popup, to_edit: String){
    app.input = to_edit;
    app.popup = popup;
    app.can_input = true;
}

fn open_delete_popup(app: &mut App, popup: Popup, to_del: String){
    app.input = to_del;
    app.popup = popup;
}

fn adjust_kanban_indexes_upon_project_change(app: &mut App){
    app.focus.chunk_focus[3] = 0;
    app.kanban.todo_index = 0;
    app.kanban.in_progress_index = 1000;
    app.kanban.done_index = 1000;
}

fn delete_in_progress(app: &mut App){
    if app.kanban.projects[app.kanban.project_index].in_progress.len() > 0{
        //remove last element
        if app.kanban.in_progress_index == app.kanban.projects[app.kanban.project_index].in_progress.len() - 1 {

            if app.kanban.in_progress_index == app.kanban.projects[app.kanban.project_index].in_progress.len() - 1 {

                app.kanban.projects[app.kanban.project_index].in_progress.remove(app.kanban.in_progress_index);

                if app.kanban.projects[app.kanban.project_index].in_progress.len() != 0 {
                    app.kanban.in_progress_index = app.kanban.projects[app.kanban.project_index].in_progress.len() - 1; 
                } else {
                    app.kanban.in_progress_index = 0;
                }

            }
        } else {
            app.kanban.projects[app.kanban.project_index].in_progress.remove(app.kanban.in_progress_index);
        }
    }
}

fn delete_done(app: &mut App){
    if app.kanban.projects[app.kanban.project_index].done.len() > 0{
        //remove last element
        if app.kanban.done_index == app.kanban.projects[app.kanban.project_index].done.len() - 1 {

            if app.kanban.done_index == app.kanban.projects[app.kanban.project_index].done.len() - 1 {

                app.kanban.projects[app.kanban.project_index].done.remove(app.kanban.done_index);

                if app.kanban.projects[app.kanban.project_index].done.len() != 0 {
                    app.kanban.done_index = app.kanban.projects[app.kanban.project_index].done.len() - 1; 
                } else {
                    app.kanban.done_index = 0;
                }

            }
        } else {
            app.kanban.projects[app.kanban.project_index].done.remove(app.kanban.done_index);
        }
    }
}

fn delete_todo(app: &mut App){
    if app.kanban.projects[app.kanban.project_index].todo.len() > 0{

        //remove last element
        if app.kanban.todo_index == app.kanban.projects[app.kanban.project_index].todo.len() - 1{
            app.kanban.projects[app.kanban.project_index].todo.remove(app.kanban.todo_index);

            if app.kanban.projects[app.kanban.project_index].todo.len() != 0{
                app.kanban.todo_index = app.kanban.projects[app.kanban.project_index].todo.len() - 1;
            }else {
                app.kanban.todo_index = 0;
            }
            
        }else {
            app.kanban.projects[app.kanban.project_index].todo.remove(app.kanban.todo_index);
        }
    }
}