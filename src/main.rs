use yew::prelude::*;

struct Model {
    value: i64
}

#[function_component(App)]
fn app() -> Html {

    let state = use_state(|| Model{
        value: 0
    });

    let onclick = {
        let state = state.clone();

        Callback::from(move |_| {
            state.set(Model {
                value: state.value + 1
            })
        })
    };

    html! {
        <div>
            <div>
                <button {onclick}>{ "+1" }</button>
            </div>
            <div>{state.value}</div>
        </div>
    }
}


fn main() {
    yew::start_app::<App>();
}
