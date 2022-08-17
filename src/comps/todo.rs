use gloo_timers::future::TimeoutFuture;
use yew::prelude::*;

use super::super::bridge::*;
use super::super::custom::CustomButton;
use super::super::data::TodoData;

use serde::{Deserialize, Serialize};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};

#[derive(Debug, Serialize, Deserialize)]
pub struct Model {
    input: TodoData,
    todos: Vec<TodoData>,

    edit_mode: bool,
    edit_item_index: usize,
}

pub enum Msg {
    Add,
    Update(TodoData),
    Remove(usize),
    RemoveAll,
    Edit(usize),
    SetAll(TodoData, Vec<TodoData>),
    UpdateEdit,

    ToggleDesc(usize),
    ToggleFinish(usize),

    Log(String),
    LogJson(String),

    FetchDatabase,
    SaveDatabase,

    Nothing,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: TodoData::new("", ""),
            todos: vec![TodoData::new("Your first ever todo!", "Add a description?")],
            edit_mode: false,
            edit_item_index: usize::MAX,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let map_value = |(index, data): (usize, &TodoData)| -> Html {
            if data.todo.trim().is_empty() && data.desc.trim().is_empty() {
                return html! {};
            }

            let mut final_class = String::from("item");

            if self.edit_mode == true && self.edit_item_index != index {
                final_class.push_str(" opacity-5");
            }

            html! {

                <li class = {final_class}>
                    <div class = "item-wrapper">
                        <div class = "item-data">
                            <textarea
                                value =  {format!("Todo {}: {}\nDate: {}", index+1, data.todo.clone(), data.time_created)}
                                class = {
                                    let mut class_data =
                                    "item-todo".to_string();

                                    if {!data.finish}{
                                        class_data
                                    }
                                    else{
                                        class_data.push_str(" item-todo-finish");
                                        class_data
                                    }

                                }
                                rows = "2"
                                readonly = true
                                id = "textarea"
                            />

                            if {data.desc.trim().is_empty() == false}{

                                if {data.toggle_desc} {

                                    <h2>{"Notes: "}</h2>

                                    <textarea value = {format!("{}", data.desc)} readonly = {true}
                                    class = "item-desc"
                                    id = "textarea"

                                    />
                                }
                            }

                        </div>

                        <div class = "item-btn">

                            if {self.edit_mode == false}{
                                <CustomButton onclick = {link.callback(move |_| Msg::Remove(index))} text = "🗑️"/>
                            }

                            <CustomButton onclick = {link.callback(move |_| Msg::Edit(index))} text = "📝"/>


                            if data.desc.trim().is_empty() == false{
                                <CustomButton onclick = {link.callback(move |_| Msg::ToggleDesc(index))} text = "🤔"/>
                            }
                            <CustomButton onclick = {link.callback(move |_| Msg::ToggleFinish(index))} text = {
                                if data.finish {
                                    "❌"
                                }
                                else{
                                    "✅"
                                }

                            } class =""/>

                        </div>
                    </div>
                </li>
            }
        };

        let on_input_field_change = link.batch_callback(|e: Event| {
            let input = e.target_dyn_into::<HtmlInputElement>();

            input.map(|input| Msg::Update(TodoData::new(&input.value().clone(), "")))
        });

        let on_text_area_change = link.batch_callback(|e: Event| {
            let input = e.target_dyn_into::<HtmlTextAreaElement>();

            input.map(|area| Msg::Update(TodoData::new("", &area.value().clone())))
        });

        let on_submit = link.callback(|_| Msg::Add);
        let on_update_submit = link.callback(|_| Msg::UpdateEdit);

        html! {
            <div class = "mp">
                <header class = "header">
                <h1 class ="">{"MULTI-BILIONS TODO APP"}</h1>
                // <p>{"desciptive stuff"}</p>



                </header>
                <div class = "input-form">
                    <div class = "inputs">
                        <input
                        value = {self.input.todo.clone()}
                        onchange = {on_input_field_change}

                        required = {true}
                        type="text"
                        placeholder="Your todo"
                        class =""
                        />

                        <textarea
                        value = {self.input.desc.clone()}
                        onchange = {on_text_area_change}

                        required = {true}
                        placeholder = "Your todo desc"
                        cols="30"
                        rows="10"
                        class = ""

                        />
                    </div>

                    <div class ="btns">
                        if {self.edit_mode == false} {
                            <button class ="submit-btn" type = "submit" onclick = {on_submit}>{"➕ Add"}</button>
                        }else{
                            <button class ="submit-btn" type = "submit" onclick = {on_update_submit}>{"👍🏻 Update"}</button>
                        }
                        <CustomButton onclick = {link.callback(|_| Msg::RemoveAll)} text = "💀 Purge" />
                        <CustomButton onclick = {link.callback(|_| Msg::FetchDatabase)} text = "📩 Load" />
                        <CustomButton onclick = {link.callback(|_| Msg::SaveDatabase)} text = "💾 Save" />
                    </div>

                    <div class ="todos">
                        <ul class = "todos-list">
                            {for self.todos.iter().enumerate().map(map_value)}
                        </ul>
                    </div>

                </div>

            </div>


        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                let mut data = self.input.clone();

                // This check is for greate messure
                // Html elements already have a required tag
                if data.todo.trim().is_empty() {
                    // There's no header or data to add
                    data.todo = String::from("To lazy to write lol");
                }

                self.todos.push(data);
                self.input.clear();
            }
            Msg::Update(data) => {
                let mut todo = data.todo;
                let mut desc = data.desc;

                if todo.trim().is_empty() {
                    todo = self.input.todo.clone();
                } else if desc.trim().is_empty() {
                    desc = self.input.desc.clone();
                }

                self.input = TodoData::new(&todo.clone(), &desc.clone());
            }
            Msg::Remove(i) => {
                self.todos.remove(i);
            }
            Msg::RemoveAll => {
                self.input.clear();
                self.todos = vec![];
            }
            Msg::Log(s) => {
                log(&s);

                return false;
            }
            Msg::Nothing => return false,
            Msg::Edit(i) => {
                self.edit_mode = true;

                let current_todo = self.todos[i].clone();

                self.edit_item_index = i;
                self.input = current_todo;
            }
            Msg::UpdateEdit => {
                self.todos[self.edit_item_index] = self.input.clone();
                self.input.clear();

                self.edit_mode = false;
            }
            Msg::ToggleDesc(i) => {
                let todo = &mut self.todos[i];

                todo.toggle_desc = !todo.toggle_desc;
            }
            Msg::ToggleFinish(i) => {
                let todo = &mut self.todos[i];

                todo.finish = !todo.finish;
            }
            Msg::LogJson(s) => {
                log_json(&s);

                return false;
            }
            Msg::SaveDatabase => {
                self.edit_mode = false;
                let data = self.to_json();
                let link = ctx.link().clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let data_uid = get_user_uid().await;

                    match data_uid {
                        Ok(s) => write_data(s.as_string().unwrap_or("user_1".to_string()), data),
                        Err(_) => write_data("user_1".to_string(), data),
                    };

                    link.send_message(Msg::Update(TodoData::new("Saved 👌🏻", "have a nice day")));
                });
            }
            Msg::FetchDatabase => {
                self.edit_mode = false;

                self.input.todo = "Fetching...🥱".to_string();
                self.input.desc =
                    "Dont worry, firebase usually quick...\n Unless my plan has expired"
                        .to_string();

                ctx.link().send_future(Model::fetch_database());
            }
            Msg::SetAll(m, v) => {
                self.input = m;
                self.todos = v;
            }
        }
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();

            wasm_bindgen_futures::spawn_local(async move {
                TimeoutFuture::new(500).await;
                // Do something here after the one second timeout is up!
                link.send_message(Msg::FetchDatabase);
            })
        }
    }
}

impl Model {
    pub fn to_json(&self) -> String {
        let json_result = serde_json::to_string_pretty(self).unwrap();

        return json_result;
    }

    pub async fn fetch_database() -> Msg {
        let data_uid = get_user_uid().await;

        let user_uid = match data_uid {
            Ok(s) => s.as_string().unwrap_or("user_1".to_string()),
            Err(_) => "user_1".to_string(),
        };

        let data: Model = take_data(user_uid).await.into_serde().unwrap_or_default();
        return Msg::SetAll(data.input, data.todos);
    }
}

impl Default for Model {
    fn default() -> Self {
        Self {
            input: TodoData::new("", ""),
            todos: vec![TodoData::new("", "")],
            edit_mode: false,
            edit_item_index: Default::default(),
        }
    }
}
