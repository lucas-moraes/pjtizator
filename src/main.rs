mod components;

use crate::components::clt::clt::Clt;
use crate::components::clt::clt::CltProps;

use crate::components::pj::pj::Pj;
use crate::components::pj::pj::PjProps;

use leptos::*;
use web_sys::HtmlInputElement;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (salary, set_salary) = create_signal(cx, String::new());
    let (vale, set_vale) = create_signal(cx, String::new());

    let format_currency = |value: &str| -> String {
        let numeric_value: String = value.chars().filter(|c| c.is_digit(10)).collect();
        let numeric_value = if numeric_value.is_empty() {
            "000".to_string()
        } else {
            format!("{:0>3}", numeric_value)
        };

        let numero: f64 = numeric_value.parse::<f64>().unwrap() / 100.0;

        // Formatando o número como moeda manualmente
        let formatted_value = format!("{:.2}", numero);
        let formatted_value = formatted_value.replace(".", ",");
        format!("R$ {}", formatted_value)
    };

    let handle_input_salary = move |ev: web_sys::Event| {
        let input_element = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let input_value = input_element.value();
        let formatted_value = format_currency(&input_value);
        input_element.set_value(&formatted_value);
        set_salary(formatted_value);
    };

    let handle_input_vale = move |ev: web_sys::Event| {
        let input_element = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let input_value = input_element.value();
        let formatted_value = format_currency(&input_value);
        input_element.set_value(&formatted_value);
        set_vale(formatted_value);
    };

    view! {cx,
            <header class="header-advise">
              <p class="header-advise-text">
                  "Atenção: Os dados são meramente ilustrativos, não substituem a consultoria de um contador."
              </p>
            </header>
            <section>
        <div class="uk-container-expand'">
            <div  class="uk-grid-small uk-grid-match" uk-grid>
              <div class="uk-child-width-1-2@m uk-flex uk-flex-middle">
                 <div class="uk-card uk-card-body ">
                 <h1>"PJtizator"</h1>
                 <p>"Calculadora comparativa entre CLT e PJ no ramo de serviços de TI."</p>
                 <fieldset class="uk-fieldset">
                    <div class="uk-margin">
                        <label class="uk-form-label" for="form-stacked-text">"Remuneração"</label>
                        <div class="uk-margin">
                            <input
                                class="uk-input
                                uk-form-width-medium"
                                type="text"
                                placeholder="R$ 0,00"
                                aria-label="Input"
                                on:input=handle_input_salary
                            />
                        </div>
                        <label class="uk-form-label" for="form-stacked-text">"Soma do Vale alimentação + Refeição"</label>
                        <div class="uk-margin">
                            <input
                                class="uk-input
                                uk-form-width-medium"
                                type="text"
                                placeholder="R$ 0,00"
                                aria-label="Input"
                                on:input=handle_input_vale
                            />
                        </div>
                      </div>
                  </fieldset>
                </div>
              </div>
              <div class="uk-child-width-1-2@m">
                <div class="uk-container-expand">
                  <div class="uk-grid-small uk-grid-match" uk-grid>
                    <div class="uk-child-width-1-2@m">
                        <Clt salary=salary vale=vale/>
                    </div>
                    <div class="uk-child-width-1-2@m">
                        <Pj invoice=salary vale=vale/>
                    </div>
                  </div>
                </div>
              </div>
           </div>
        </div>
        </section>
        <footer class="footer-advise">
            <p class="footer-advise-text">
                "Feito com ❤️ em Rust"
            </p>
        </footer>

    }
}

fn main() {
    mount_to_body(move |cx| view! {cx, <App/>});
}
