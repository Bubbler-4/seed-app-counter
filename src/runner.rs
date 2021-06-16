use std::cell::RefCell;
use std::rc::Rc;
use wasm_mt::prelude::*;
use wasm_mt::Thread;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use seed::log;

struct RunnerState {
    mt: Option<WasmMt>,
    thread: Option<Thread>,
    th_init: bool,
    result: Option<(String, String)>,
}

static mut STATE2: RunnerState = RunnerState {
    mt: None,
    thread: None,
    th_init: false,
    result: None
};

fn set_mt(mt: WasmMt) {
    unsafe {
        STATE2.mt = Some(mt);
    }
}

fn get_mt() -> Option<&'static WasmMt> {
    unsafe {
        STATE2.mt.as_ref()
    }
}

fn reset_thread() {
    unsafe {
        STATE2.thread = None;
    }
}

fn set_thread(th: Thread) {
    unsafe {
        STATE2.thread = Some(th);
    }
}

fn get_thread() -> Option<&'static Thread> {
    unsafe {
        STATE2.thread.as_ref()
    }
}

fn set_th_init(b: bool) {
    unsafe {
        STATE2.th_init = b;
    }
}

fn get_th_init() -> bool {
    unsafe {
        STATE2.th_init
    }
}

fn reset_result() {
    unsafe {
        STATE2.result = None;
    }
}

fn set_result(s1: String, s2: String) {
    unsafe {
        STATE2.result = Some((s1, s2));
    }
}

fn get_result() -> Option<(String, String)> {
    unsafe {
        STATE2.result.clone()
    }
}

thread_local!{
    static STATE: Rc<RefCell<RunnerState>> = Rc::new(RefCell::new(RunnerState {
        mt: None,
        thread: None,
        th_init: false,
        result: None,
    }));
}

pub fn runner_init() {
    spawn_local(async {
        let pkg_js = "./pkg/package.js";
        let mt = WasmMt::new(pkg_js).and_init().await.unwrap();
        let th = mt.thread().and_init().await.unwrap();
        log!("success");
        set_mt(mt);
        set_thread(th);
        log!("success 2");
    });
}

pub fn poll_thread_init() -> bool {
    if get_thread().is_some() && !get_th_init() {
        log!("thread init success");
        set_th_init(true);
    }
    get_th_init()
}

pub fn run() {
    reset_result();
    spawn_local(async {
        let thread = get_thread().unwrap();
        let result = exec!(thread, || computation()).await;
        //let result = exec!(thread, || runner_panics()).await;
        log!(result);
        match result {
            Ok(jsval) => {
                let (s1, s2): (String, String) = jsval.into_serde().unwrap();
                set_result(s1, s2);
            }
            Err(_) => {
                set_result("".to_string(), "err found".to_string());
            }
        }
    });
}

pub fn poll_thread_result() -> Option<(String, String)> {
    get_result()
}

pub fn reset() {
    let thread = get_thread().unwrap();
    thread.terminate();
    reset_thread();
    spawn_local(async {
        let mt = get_mt().unwrap();
        let th = mt.thread().and_init().await.unwrap();
        log!("reset success");
        set_thread(th);
        log!("reset success 2");
    });
}

fn computation() -> Result<JsValue, JsValue> {
    let mut s: String = String::new();
    for i in 0..400000000 {
        if i % 1000000 == 0 {
            s += "S";
        }
    }
    Ok(JsValue::from_serde(&(s, "".to_string())).unwrap())
}

#[allow(dead_code)]
fn runner_panics() -> Result<JsValue, JsValue> {
    let v: Vec<usize> = vec![];
    let n = v[0];
    log!(n);
    loop {}
}