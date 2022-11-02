mod owner;
mod pet;

pub type Anchor = RouterAnchor<AppRoute>;

struct FullStackApp {}

pub enum Msg {}

#[derive(Switch, Clone, Debug)]
pub enum AppRoute {
    #[to = "/app/create-owner"]
    CreateOwner,
    #[to = "/app/create-pet/{id}"]
    CreatePet(i32),
    #[to = "/app/{id}"]
    Detail(i32),
    #[to = "/"]
    Home,
}

impl Component for FullStackApp {
    type Message = Msg;
    type Properties = ();
    
    fn create(
        _: Self::Properties,
        _link: ComponentLink<Self>,
    ) -> Self {
        Self {}
    }

    fn update(
        &mut self,
        _msg: Self::Message,
    ) -> ShouldRender {
        true
    }

    fn change(
        &mut self,
        _props: Self::Properties,
    ) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html!{
            <div class=classes!("app")>
                <div class=classes!("nav")>
                    <Anchor route=AppRoute::Home>{"Home"}</Anchor>
                </div>
                <div class=classes!("content")>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute| {
                            match switch {
                                AppRoute::CreateOwner => {
                                    html!{
                                        <div>
                                            <owner::create::CreateForm />
                                        </div>
                                    }
                                }
                                AppRoute::CreatePet(owner_id) => {
                                    html!{
                                        <div>
                                            <pet::create::OwnerForm owner_id=owner_id />
                                        </div>
                                    }
                                }
                                AppRoute::Detail(owner_id) => {
                                    html!{
                                        <div>
                                            <owner::detail::Detail owner_id=owner_id />
                                        </div>
                                    }
                                }
                                AppRoute::Home => {
                                    html!{
                                        <div>
                                            <owner::list::List />
                                            <br />
                                            <Anchor route=AppRoute::CreateOwner>
                                                { "Create new Owner" }
                                            </Anchor>
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
    App::<FullStackApp>::new().mount_to_body();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
