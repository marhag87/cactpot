mod components;
mod logic;

use components::grid::Grid;
use components::payout_table::{PayoutTable, SortBy};
use logic::Board;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    // Handles
    let board = use_state(|| Board::default());
    let sort_by = use_state(|| SortBy::Max);

    // Callbacks
    let on_clear = {
        let board = board.clone();
        Callback::from(move |idx: usize| {
            let mut new_board = (*board).clone();
            new_board.cells[idx] = None;
            board.set(new_board);
        })
    };

    let on_reset = {
        let board = board.clone();
        Callback::from(move |_| {
            board.set(Board::default());
        })
    };

    let set_sort = {
        let sort_by = sort_by.clone();
        Callback::from(move |sort: SortBy| {
            sort_by.set(sort);
        })
    };

    html! {
        <div class={classes!("cactpot-vertical-center")}>
            <div class={classes!("cactpot-flex")}>
                <div class={classes!("cactpot-grid-container")}>
                    <Grid
                        board_handle={board.clone()}
                        sort_by={*sort_by}
                        on_clear={on_clear.clone()}
                    />
                </div>
                <div class={classes!("cactpot-table-container")}>
                    <PayoutTable
                        board_handle={board.clone()}
                        sort_by={*sort_by}
                        set_sort={set_sort.clone()}
                    />
                </div>
            </div>
            <button onclick={on_reset} class={classes!("cactpot-reset-btn")}>{"Reset"}</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
