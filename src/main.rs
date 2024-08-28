use yew::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <TabBar/>
            <h1>{ "Yipee" }</h1>
            <CoolButton/>
        </>
    }
}

#[function_component]
fn CoolButton() -> Html {
    html! {
        <>
            <div class="buttons">
              <button class="button is-primary is-light">{"Primary"}</button>
              <button class="button is-link is-light">{"Link"}</button>
            </div>
            
            <div class="buttons">
              <button class="button is-info is-light">{"Info"}</button>
              <button class="button is-success is-light">{"Success"}</button>
              <button class="button is-warning is-light">{"Warning"}</button>
              <button class="button is-danger is-light">{"Danger"}</button>
            </div>
        </>
    }
}

#[function_component]
fn TabBar() -> Html {
    let tabs_state = use_state(|| [true, false, false, false]);
    let tabs = ["Feed", "Chat", "Following", "Followers"];
    let update_bar = |status: [bool; 4]| {
         let tabs_state = tabs_state.clone();
         Callback::from(move |_| tabs_state.set(status))
    };

    html! {
        <div class="tabs is-boxed">
          <ul>{
            tabs.iter().enumerate().map(|(i, name)| {
                let mut status = [false; 4];
                status[i] = true;

                html! {
                    <li onclick={ update_bar(status) } class={ if tabs_state[i] {"is-active"} else {""} }>
                        <a> <span>{name}</span> </a>
                    </li>
                }
            }).collect::<Html>()
          }</ul>
        </div>
    }
}
