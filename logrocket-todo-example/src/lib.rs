use serde::{Deserialize};
use wasm_bindgen::prelude::*;
use yew::{App, Component, ComponentLink, format::{Json, Nothing}, html, ShouldRender};
use yew::prelude::*;
use yew::services::{ConsoleService, fetch::{FetchTask, Request, Response}, FetchService};
use yew_router::{components::RouterAnchor, router::Router, Switch};

mod todo;

pub type Anchor = RouterAnchor<AppRoute>;

struct TodoApp {
    link: ComponentLink<Self>,
    todos: Option<Vec<Todo>>,
    fetch_task: Option<FetchTask>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub user_id: i64,
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

enum Msg {
    MakeReq,
    Resp(Result<Vec<Todo>, anyhow::Error>)
}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/todo/{id}"]
    Detail(i32),
    #[to = "/"]
    Home,
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);
        Self {
            link,
            todos: None,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                self.todos = None;
                let req = Request::get("https://jsonplaceholder.typicode.com/todos")
                .body(Nothing)
                .expect("Can make request to placeholder");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<Todo>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("Can create task");
                self.fetch_task = Some(task);
                ()
            }

            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.todos = Some(data);
                }
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let todos = self.todos.clone();
        let cb = self.link.callback(|_| Msg::MakeReq);
        ConsoleService::info(&format!("render TodoApp: {:?}", todos));
        html! {
            <div class=classes!("todo")>
                <div class=classes!("nav")>
                    <Anchor route=AppRoute::Home>{ "Home" }</Anchor>
                </div>
                <div class=classes!("content")>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute| {
                            match switch {
                                AppRoute::Detail(todo_id) => {
                                    html! {
                                        <div>
                                            <todo::detail::Detail todo_id=todo_id/>
                                        </div>
                                    }
                                }
                                AppRoute::Home => {
                                    html! {
                                        <div>
                                            <div class=classes!("refresh")>
                                                <button onclick=cb.clone()>
                                                    { "refresh" }
                                                </button>
                                            </div>
                                            <todo::list::List todos=todos.clone()/>
                                        </div>
                                    }
                                }
                            }
                        })
                    />
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<TodoApp>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
