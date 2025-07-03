use crate::{
    components::payout_table::SortBy,
    logic::{Board, MAX_NUM, MIN_NUM, NUM_CELLS},
};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GridProps {
    pub board_handle: UseStateHandle<Board>,
    pub sort_by: SortBy,
    pub on_clear: Callback<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScrollDirection {
    Up,
    Down,
}

impl From<f64> for ScrollDirection {
    fn from(delta: f64) -> Self {
        if delta < 0.0 {
            ScrollDirection::Up
        } else {
            ScrollDirection::Down
        }
    }
}

fn next_number(board: &Board, idx: usize, direction: ScrollDirection) -> Option<u8> {
    let current = board.cells[idx];
    let used = board.used();
    let unused = (MIN_NUM..=MAX_NUM)
        .filter(|n| !used.contains(n) || Some(*n) == current)
        .collect::<Vec<u8>>();
    match current {
        Some(n) => {
            let pos = if let Some(p) = unused.iter().position(|&x| x == n) {
                p
            } else {
                return current;
            };
            match direction {
                ScrollDirection::Up => {
                    if pos + 1 < unused.len() {
                        Some(unused[pos + 1])
                    } else {
                        Some(n)
                    }
                }
                ScrollDirection::Down => {
                    if pos > 0 {
                        Some(unused[pos - 1])
                    } else {
                        Some(n)
                    }
                }
            }
        }
        None => match direction {
            ScrollDirection::Up => unused.iter().min().copied(),
            ScrollDirection::Down => unused.iter().max().copied(),
        },
    }
}

#[function_component(Grid)]
pub fn grid(props: &GridProps) -> Html {
    let board = (*props.board_handle).clone();
    let numbers = board.cells;
    let on_wheel = {
        let board_handle = props.board_handle.clone();
        Callback::from(move |(idx, direction): (usize, ScrollDirection)| {
            let mut board = (*board_handle).clone();
            let next = next_number(&board, idx, direction);
            board.cells[idx] = next;
            board_handle.set(board);
        })
    };

    let max_inputs_reached = board.max_inputs_reached();
    let best_line_cells = board.best_line_cells(props.sort_by);

    html! {
        <div class={classes!("cactpot-grid")}>
            { (0..NUM_CELLS).map(|i| {
                let value = numbers[i];
                let has_value = value.is_some();
                let max_inputs_reached = max_inputs_reached;
                let onwheel = {
                    let on_wheel = on_wheel.clone();
                    Callback::from(move |e: web_sys::WheelEvent| {
                        e.prevent_default();
                        if !max_inputs_reached || has_value {
                            let direction: ScrollDirection = e.delta_y().into();
                            on_wheel.emit((i, direction));
                        }
                    })
                };
                let oncontextmenu = {
                    let on_clear = props.on_clear.clone();
                    Callback::from(move |e: web_sys::MouseEvent| {
                        e.prevent_default();
                        if !max_inputs_reached || has_value {
                            on_clear.emit(i);
                        }
                    })
                };
                let is_best = best_line_cells.is_some_and(|line| line.contains(&i));
                let mut class = classes!("cactpot-cell");
                if is_best { class.push("cactpot-best-cell"); }
                if has_value { class.push("cactpot-cell-revealed"); }
                html! {
                    <div {class} tabindex="0" {onwheel} {oncontextmenu}>
                        { value.map(|n| n.to_string()).unwrap_or_default() }
                    </div>
                }
            }).collect::<Html>() }
        </div>
    }
}
