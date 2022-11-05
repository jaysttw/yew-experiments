pub struct CreateForm {
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    state_name: String,
}

pub enum Msg {
    MakeReq,
    Resp(Result<OwnerResponse, anyhow::Error>),
    EditName(String),
}

implement Component for CreateForm {
    type Properties = ();
    type Message = Msg;

    fn create(
        _props: Self::Properties,
        link: ComponentLink<Self>,
    ) -> Self {
        Self {
            link,
            state_name: String::new(),
            fetch_task: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_form() }
            </div>
        }
    }

    fn update(
        &mut self,
        msg: Self::Message
    ) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                let body = OwnerRequest {
                    name = self.state_name.clone(),
                };
                
                let req = Request::post("http://localhost:8000/owner")
                    .header("Content-Header", "application/json")
                    .body(Json(&body))
                    .expect("Can make request to backend");

                let cb = self.link.callback(
                    |response: Response<Json<Result<OwnerResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    }
                )

                let task = FetchService::fetch(req, cb).expect("Can create task");
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                ConsoleService::info(
                    &format!("Owner created: {:?}", resp)
                )
                if let Ok(_) = resp {
                    RouteAgent::dispatcher()
                        .send(RouteRequest::ChangeRoute(
                            Route {
                                route: "/".to_string(),
                                state: (),
                            }
                        ));
                }
            }
            Msg::EditName(input) => {
                self.state_number = input;
            }
        }
        true
    }

    fn change(
        &mut self,
        _props: Self::Properties,
    ) -> ShouldRender {
        true
    }
}

impl CreateForm {
    fn render_form(&self) -> Html {
        let edit_name = self
            .link
            .callback(move |e: InputData| Msg::EditName(e.value));

        html! {
            <div class=classes!("pet-form")>
                <div>
                    <input type="text" value={self.state_name.clone()} oninput={edit_name} />
                </div>
                <div>
                    <button onclick=self.link.callback(
                        move |_| Msg::MakeReq
                    )>
                        {"Submit"}
                    </button>
                </div>
            </div>
        }
    }
}