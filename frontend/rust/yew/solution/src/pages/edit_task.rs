use crate::components::atoms::bb_textarea::BBTextarea;
use crate::router::Route;
use crate::{
    components::atoms::bb_text_input::{BBTextInput, InputType},
    store::StoreType,
};
use gloo::console::log;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::{history, prelude::*};
use yewdux_functional::use_store;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub id: u32,
}

#[styled_component(EditTask)]
pub fn edit_task(props: &Props) -> Html {
    let stylesheet = css!(r#""#);

    let title_onchange = Callback::from(|title| {
        log!(title);
    });
    let description_onchange = Callback::from(|description| {
        log!(description);
    });
    let task = use_store::<StoreType>()
        .state()
        .map(|store| store.get_task_by_id(props.id))
        .unwrap_or_default();

    let (task_title, task_description) = match task {
        Some(task) => (task.title, task.description.unwrap_or_default()),
        None => (String::new(), String::new()),
    };

    html! {
      <section class={stylesheet}>
        <form>
          <BBTextInput data_test="editing-title" label="Task Title" input_type={InputType::Text} onchange={title_onchange} value={task_title} />
          <BBTextarea
            data_test="editing-description"
            value={task_description}
            onchange={description_onchange}
            label="Edit Description"
            id={format!("description-{}", props.id)}
          />
        </form>
      </section>
    }
}