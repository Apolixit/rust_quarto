use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ButtonProps<T>
    where T: PartialEq {
    title: String,
    callback: Callback<T>
}

// #[function_component(ActionButton)]
// fn action_button<T>(p: &ButtonProps<T>) -> Html
//     where T: PartialEq {
//     //<button type="button" class="basis-1/4 bg-blue-400" onclick={ctx.link().callback(|_| AppMessage::NewPvpGame)}>{ "Start game vs Human" }</button>
//     html! {
//         <button type="button" class="basis-1/4 bg-blue-400" onclick={p.callback.clone()}>{ &p.title }</button>
//     }
// }