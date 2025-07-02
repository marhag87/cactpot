use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GridProps {
    pub set_numbers: UseStateHandle<Vec<Option<u8>>>,
    pub best_line_cells: Option<[usize; 3]>,
    pub on_clear: Callback<usize>,
    pub max_inputs_reached: bool,
}

const NUM_CELLS: usize = 9;
const MIN_NUM: u8 = 1;
const MAX_NUM: u8 = 9;

#[function_component(Grid)]
pub fn grid(props: &GridProps) -> Html {
    let numbers = (*props.set_numbers).clone();
    let on_wheel = {
        let set_numbers = props.set_numbers.clone();
        Callback::from(move |(idx, delta): (usize, i32)| {
            let mut nums = (*set_numbers).clone();
            let current = nums[idx];
            let used: Vec<u8> = nums.iter().filter_map(|&n| n).collect();
            let unused: Vec<u8> = (MIN_NUM..=MAX_NUM)
                .filter(|n| !used.contains(n) || Some(*n) == current)
                .collect();
            if unused.is_empty() {
                return;
            }
            let next = match current {
                Some(n) => {
                    let pos = unused.iter().position(|&x| x == n).unwrap();
                    if delta < 0 {
                        // Scroll up: increment
                        if pos + 1 < unused.len() {
                            Some(unused[pos + 1])
                        } else {
                            Some(n) // At max, do nothing
                        }
                    } else {
                        // Scroll down: decrement
                        if pos > 0 {
                            Some(unused[pos - 1])
                        } else {
                            Some(n) // At min, do nothing
                        }
                    }
                }
                None => {
                    if delta < 0 {
                        // Scroll up: set to lowest unused
                        Some(*unused.iter().min().unwrap())
                    } else {
                        // Scroll down: set to highest unused
                        Some(*unused.iter().max().unwrap())
                    }
                }
            };
            nums[idx] = next;
            set_numbers.set(nums);
        })
    };

    html! {
        <div class={classes!("cactpot-grid")}>
            { (0..NUM_CELLS).map(|i| {
                let value = numbers[i];
                let is_empty = value.is_none();
                let onwheel = if !props.max_inputs_reached || !is_empty {
                    let on_wheel = on_wheel.clone();
                    Callback::from(move |e: web_sys::WheelEvent| {
                        e.prevent_default();
                        let delta = e.delta_y() as i32;
                        on_wheel.emit((i, delta));
                    })
                } else {
                    Callback::from(|e: web_sys::WheelEvent| {
                        e.prevent_default();
                    })
                };
                let oncontextmenu = if !props.max_inputs_reached || !is_empty {
                    let on_clear = props.on_clear.clone();
                    Callback::from(move |e: web_sys::MouseEvent| {
                        e.prevent_default();
                        on_clear.emit(i);
                    })
                } else {
                    Callback::from(|e: web_sys::MouseEvent| {
                        e.prevent_default();
                    })
                };
                let is_best = props.best_line_cells.map_or(false, |line| line.contains(&i));
                let mut cell_class = vec!["cactpot-cell"];
                if is_best { cell_class.push("cactpot-best-cell"); }
                if value.is_some() { cell_class.push("cactpot-cell-revealed"); }
                let number_class = if value.is_some() { Some("cactpot-cell-filled") } else { None };
                html! {
                    <div class={classes!(cell_class)}
                         tabindex="0"
                         onwheel={onwheel}
                         oncontextmenu={oncontextmenu}>
                        <span class={number_class}>{ value.map(|n| n.to_string()).unwrap_or_default() }</span>
                    </div>
                }
            }).collect::<Html>() }
        </div>
    }
}
