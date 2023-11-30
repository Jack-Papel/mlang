#[allow(unused_imports)]
use log::{error, info, warn};
use mlang_interpreter::program::Program;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

fn run_program(
    code: UseStateHandle<String>,
    output: UseStateHandle<String>,
    error_occurred: UseStateHandle<bool>,
) {
    if let Ok(program) = Program::new((*code).clone()) {
        output.set(match program.parse_and_run() {
            Ok(out) => {
                error_occurred.set(false);
                out
            }
            Err(out) => {
                error_occurred.set(true);
                out
            }
        })
    } else {
        error_occurred.set(true);
        error!("An unexpected error has occured.");
        output.set(String::from("An unexpected error has occured."));
    }
}

#[function_component]
fn App() -> Html {
    let code = use_state(|| {
        String::from(
r#""Printing the prime numbers from 1 - 50, squared:" println

let is_prime = | num ~ num < 2 : false
               | num : 2..(num - 1) &&& 
                 | factor : num % factor != 0

0..50
    # is_prime 
    @ (| p : p * p)
    $ println

"\n" print
"Now doing fizzbuzz up to 20:" println

1..20 $ | num :
          let out = ""
          (|~ num % 3 == 0: out = out + "fizz")
          (|~ num % 5 == 0: out = out + "buzz")
          (|~ out == "": out = num)
          out println
"#,
        )
    });
    let output = use_state(String::new);
    let error_occurred = use_state(|| false);

    let onclick = {
        let code = code.clone();
        let output = output.clone();
        let error_occurred = error_occurred.clone();
        move |_| run_program(code.clone(), output.clone(), error_occurred.clone())
    };

    let onchange = {
        let code = code.clone();
        Callback::from(move |e: Event| {
            let target = e.target().and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok()).unwrap();
            code.set(target.value());
        })
    };

    let onkeydown = {
        let code = code.clone();
        Callback::from(move |e: KeyboardEvent| {
            let target = e.target().and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok()).unwrap();
            let string = (*code).clone();
            if let Some((start, end)) = target.selection_start().unwrap().zip(target.selection_end().unwrap()) {
                let left = string.split_at(start as usize).0;
                let right = string.split_at(end as usize).1;
                if e.key() == "Tab" {
                    e.prevent_default();
                    code.set(String::from(left) + "  " + right);
                }
            }
        })
    };

    html! {
        <>
            <nav class="shortcuts" style="--scroll:1; position:unset">
                <ul>
                <li><a href={"https://www.github.com/jack-papel/mlang"}>{"Github"}</a></li>
                    <li><a href={"https://www.jackpapel.com"}>{"Author"}</a></li>
                    <li><a href={"https://www.jackpapel.com/#portfolio"}>{"Other projects"}</a></li>
                </ul>
            </nav>
            <main>
                <section id={"about"}>
                    <div class={"center"}>
                        <h2>{"Mlang Online Playground"}</h2>
                        <p>
                        {r#"This is a fun side project I started. I often have fun ideas for languages, and I decided for this project I'd learn how to make one. 
                            The premise of this language is that match statements are objects. For more information (and a tutorial), check out the "#}
                            <a href="https://www.github.com/Jack-Papel/mlang">{"github"}</a>
                            {" page for it."}
                            <br/><br/>
                        {r#"I was recommended to use Lex and Yacc, but I didn't want to install C/C++ tools, so I made it in rust. 
                            Because of this, it's not a compiled language - that would be a lot of work."#}
                        </p>
                    </div>
                </section>
                <section id={"playground"}>
                    <nav>
                        <button {onclick}>{"Run"}</button>
                    </nav>
                    <div class="split">
                        <div>
                            <label for="code">{"Write code here:"}</label>
                            <textarea name="code" spellcheck="false" {onchange} {onkeydown} value={(*code).clone()}>
                            </textarea>
                        </div>
                        <div>
                            <label for="code">{"Output:"}</label>
                            <textarea name="output" spellcheck="false" disabled={true} error={(*error_occurred).to_string()} value={(*output).clone()}>
                            </textarea>
                        </div>
                    </div>
                </section>
            </main>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::Renderer::<App>::new().render();
}
