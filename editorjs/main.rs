#![recursion_limit = "1024"]

use console_error_panic_hook::set_once as set_panic_hook;

mod app;
use app::Body;

fn main() {
    set_panic_hook();

    yew::start_app::<Body>();
}
