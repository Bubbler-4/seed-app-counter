#![allow(clippy::wildcard_imports)]

mod runner;

use seed::{prelude::*, *};

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.after_next_render(Msg::Rendered);
    runner::runner_init();
    Model {
        cnt: 0,
        spinner: 0,
        thread_ready: false,
        thread_running: false,
        stdout: String::new(),
        stderr: String::new(),
    }
}

struct Model {
    cnt: i32,
    spinner: i32,
    thread_ready: bool,
    thread_running: bool,
    stdout: String,
    stderr: String,
}

enum Msg {
    Increment,
    Rendered(RenderInfo),
    Run,
    Stop,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Increment => model.cnt += 1,
        Msg::Rendered(_) => {
            model.spinner += 1;
            if !model.thread_ready {
                model.thread_ready = runner::poll_thread_init();
            }
            if model.thread_running {
                let res = runner::poll_thread_result();
                log!("result received:", res);
                model.thread_running = res.is_none();
                if let Some((s1, s2)) = res {
                    model.stdout = s1;
                    model.stderr = s2;
                }
            }
            orders.after_next_render(Msg::Rendered);
        }
        Msg::Run => {
            log!("Run clicked");
            model.thread_running = true;
            model.stdout.clear();
            model.stderr.clear();
            runner::run();
        }
        Msg::Stop => {
            log!("Stop clicked");
            model.thread_ready = false;
            model.thread_running = false;
            model.stdout.clear();
            model.stderr.clear();
            model.stderr += "aborted";
            runner::reset();
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    div![
        C!["counter"],
        "This is a counter: ",
        button![model.cnt, ev(Ev::Click, |_| Msg::Increment),],
        ".".repeat(model.spinner as usize / 10 % 10),
        br![], br![],
        button![
            attrs!{ At::Disabled => (!model.thread_ready || model.thread_running).as_at_value() },
            "Run!",
            ev(Ev::Click, |_| Msg::Run),
        ],
        button![
            attrs!{ At::Disabled => (!model.thread_ready || !model.thread_running).as_at_value() },
            "Stop!",
            ev(Ev::Click, |_| Msg::Stop),
        ],
        br![], br![],
        "stdout",
        div![&model.stdout],
        "stderr",
        div![&model.stderr],
    ]
}

#[wasm_bindgen]
pub fn start() {
    App::start("app", init, update, view);
}
