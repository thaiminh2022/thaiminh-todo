use chrono::*;
use gloo::timers::callback::Interval;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use super::super::custom::CustomButton;

pub struct Counter {
    count_time: i64,
    is_count_down: bool,

    started_counting: bool,

    interval: Option<Interval>,
    options_active: bool,
}

pub enum Msg {
    StartCount,
    StopCount,

    ToggleTimerType,
    ToggleActive,

    Update(usize, String),

    Tick,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            count_time: 0,
            is_count_down: true,

            started_counting: false,

            interval: None,
            options_active: false,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let start_count = link.callback(|_| Msg::StartCount);
        let stop_count = link.callback(|_| Msg::StopCount);
        let toggle_count_down = link.callback(|_: MouseEvent| Msg::ToggleTimerType);

        let toggle_on_top = link.callback(|_| Msg::ToggleActive);

        // Convert current seconds to 00:00:00 format, split it to get each item,
        let data_time = NaiveDateTime::from_timestamp(self.count_time, 0)
            .time()
            .to_string();
        let time_vec: Vec<String> = data_time.split(":").map(|f| f.to_string()).collect();

        let time_closure = |(index, num): (usize, &String)| -> Html {
            let on_input_field_change = link.batch_callback(move |e: Event| {
                let input = e.target_dyn_into::<HtmlInputElement>();

                input.map(|input| Msg::Update(index, input.value()))
            });

            html! {
                // Create each item as h1 for better styling in small screen
                <input
                type="number"
                class = "time-input"
                readonly = {self.started_counting}
                onchange = {on_input_field_change}
                value = {num.to_string()}
                />

            }
        };

        html! {
            <div class="ct">
                // Main for time
                <div class="header">
                    {for time_vec.iter().enumerate().map(time_closure)}
                </div>

                <button onclick = {toggle_on_top.clone()} class = {
                    let mut final_class = String::from("black-overlay");

                    if self.options_active == false{
                        final_class.push_str(" none-overlay");
                    }

                    final_class

                }></button>

                // <CustomButton extra_class = "open-options" onclick = {toggle_on_top.clone()} text = "+"/>

                <div hidden = {!self.options_active} class="options">
                    <div class="option-wrapper">

                        <div  class ="count-type-wrapper">
                            <input id ="count-type-toggle" type="checkbox" checked = {self.is_count_down} onclick = {toggle_count_down}/>
                            <label for="count-type-toggle">{
                                if self.is_count_down == false{
                                    "Counting up!!"
                                }
                                else{
                                    "Counting down!!"
                                }


                            }</label>
                        </div>


                        <div class="btns">
                           <CustomButton onclick = {start_count} text = "Start Count"/>
                           <CustomButton onclick  = {stop_count} text = "Stop Count" />
                        </div>
                    </div>

                </div>


            </div>

        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::StartCount => {
                self.cancle();

                let handle = {
                    let link = ctx.link().clone();
                    Interval::new(1000, move || link.send_message(Msg::Tick))
                };
                self.interval = Some(handle);
                self.started_counting = true;

                self.options_active = false;
            }
            Msg::StopCount => {
                if self.interval.is_some() {
                    self.cancle();
                    self.started_counting = false;
                }
            }
            Msg::Tick => {
                if self.is_count_down {
                    self.count_time -= 1;

                    if self.count_time <= 0 {
                        gloo::console::log!("FINISH!");

                        self.cancle();
                        self.count_time = 0;
                    }
                } else {
                    self.count_time += 1;
                }
            }
            Msg::ToggleTimerType => {
                self.is_count_down = !self.is_count_down;
            }
            Msg::ToggleActive => {
                self.options_active = !self.options_active;
            }
            Msg::Update(i, s) => {
                let default_num = self.count_time.clone() as f64;
                let mut final_num = 0_f64;
                let mut num: f64 = s.parse().unwrap_or_default();
                num = num.abs();

                // ! TODO: Find a cleaner way of doing this
                if i == 0 {
                    // hour time, 1hrs = 3600 sec
                    if num > 23.0 {
                        num = 23.0;
                    }
                    let left_over = (default_num / 3600.0) % 1.0;
                    final_num = (num + left_over) * 3600.0;
                } else if i == 1 {
                    // minute time, 1 minute = 60sec
                    if num > 59.0 {
                        // Clamp value
                        num = 59.0;
                    }

                    let current_hour = (default_num / 3600.0).floor(); //hour
                    let current_second = ((default_num / 3600.0) % 1.0) * 3600.0; // seconds
                    let left_over_second =
                        current_second - ((current_second / 60.0).floor() * 60.0);

                    final_num = current_hour * 3600.0 + num * 60.0 + left_over_second;
                } else if i == 2 {
                    // seconds, clamp only
                    if num > 59.0 {
                        num = 59.0;
                    }

                    let current_minute = (default_num / 60.0).floor(); // get the minute but not the leftover second
                    final_num = current_minute * 60.0 + num;
                }

                self.count_time = final_num as i64;
            }
        }

        return true;
    }
}

impl Counter {
    fn cancle(&mut self) {
        self.interval = None;
    }
}
