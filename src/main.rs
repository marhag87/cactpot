use yew::prelude::*;

const GRID_SIZE: usize = 3;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class={classes!("cactpot-center")}> 
            <div class={classes!("cactpot-grid")}> 
                { (0..GRID_SIZE * GRID_SIZE).map(|_| html! {
                    <div class={classes!("cactpot-cell")}> 
                        { "" }
                    </div>
                }).collect::<Html>() }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
