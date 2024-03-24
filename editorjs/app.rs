use web_sys::js_sys;
use yew::html::InputData;
use yew::prelude::*;
use wasm_bindgen::prelude::*;

pub struct Body {
    js_input: String,
    js_output: Vec<String>,
    line_number: usize,
    link: ComponentLink<Self>,
}

pub enum Msg {
    UpdateJsInput(String),
    RunJs,
}

impl Body {
    fn run_js(&self) -> Result<Vec<String>, JsValue> {
        let js_code = format!(
            "
            (function() {{
                let console_log_output = '';
                let original_console_log = console.log;
                console.log = function(message) {{
                    console_log_output += message + '\\n';
                    original_console_log(message);
                }};
                let result = (async function() {{
                    {}
                }})();
                if (result instanceof Promise) {{
                    result.then(function(value) {{
                        console.log(value);
                    }});
                }}
                console.log = original_console_log;
                return console_log_output;
            }})()
        ",
            self.js_input
        );
        let result = js_sys::eval(&js_code);
        let output = match result {
            Ok(js_value) => {
                js_value.as_string().unwrap_or_else(|| "Resultado não é uma string".to_string())
            }
            Err(e) => {
                format!("Erro: {:?}", e)
            }
        };
        Ok(vec![output])
    }
}

impl Component for Body {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            js_input: "".into(),
            js_output: Vec::new(),
            line_number: 1,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateJsInput(input) => {
                self.js_input = input;
                self.line_number = self.js_input.lines().count().max(3000);
                true
            }
            Msg::RunJs => {
                self.js_output.clear();
                let result = self.run_js();
                match result {
                    Ok(outputs) => {
                        for output in outputs {
                            self.js_output.push(format!("> {}", output));
                            self.line_number += 1;
                        }
                    }
                    Err(_) => {}
                }
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        let line_numbers: Vec<_> = (1..=3000).map(|n| html! { <div>{n}</div> }).collect();
        html! {
            <div class="flex gap-0 h-screen">
                <aside class="bg-[#333333] p-4  flex flex-col items-center w-60">
                    <button class="w-full h-10 btn" onclick=self.link.callback(|_| Msg::RunJs)>{"Run"}</button>
                </aside>
                <div class="flex flex-col w-full">
                    <div class="flex" style="height: 50%;">
                        <div class="line-numbers flex w-full overflow-auto bg-zinc-800 border border-black " style="line-height: 1.5;">
                            <div class="flex flex-col items-center py-2 pl-2 pr-1 text-white text-sm border-r border-black">{for line_numbers}</div>
                            <div class="outline-0 p-2 text-white text-sm leading-5  w-full leading-4" 
                                oninput=self.link.callback(|e: InputData| Msg::UpdateJsInput(e.value))
                                value=self.js_input.clone() 
                                placeholder={"Write your JavaScript here..."} contenteditable="true" />
                        </div>
                    </div>
                    <div class="h-full  p-2 border bg-zinc-900 text-white border-black">
                        {for self.js_output.iter().map(|line| html! {<p class="break-all">{line}</p>})}
                    </div>
                </div>
            </div>
        }
    }
}
