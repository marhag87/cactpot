use yew::prelude::*;

const GRID_SIZE: usize = 3;
const NUM_CELLS: usize = GRID_SIZE * GRID_SIZE;
const MIN_NUM: u8 = 1;
const MAX_NUM: u8 = 9;

#[function_component(App)]
fn app() -> Html {
    let numbers = use_state(|| vec![None; NUM_CELLS]);

    let on_wheel = {
        let numbers = numbers.clone();
        Callback::from(move |(idx, delta): (usize, i32)| {
            let mut nums = (*numbers).clone();
            let current = nums[idx];
            let used: Vec<u8> = nums.iter().filter_map(|&n| n).collect();
            let mut next = match current {
                Some(n) => n,
                None => MIN_NUM - 1, // So scrolling up starts at 1
            };
            let mut found = false;
            for _ in 0..(MAX_NUM - MIN_NUM + 2) { // +2 to ensure wrap
                if delta < 0 {
                    // Scroll up: increment
                    next = if next >= MAX_NUM { MIN_NUM } else { next + 1 };
                } else {
                    // Scroll down: decrement
                    next = if next <= MIN_NUM { MAX_NUM } else { next - 1 };
                }
                if !used.contains(&next) || Some(next) == current {
                    found = true;
                    break;
                }
            }
            if found {
                nums[idx] = Some(next);
            }
            numbers.set(nums);
        })
    };

    let on_clear = {
        let numbers = numbers.clone();
        Callback::from(move |idx: usize| {
            let mut nums = (*numbers).clone();
            nums[idx] = None;
            numbers.set(nums);
        })
    };

    html! {
        <div class={classes!("cactpot-center")}> 
            <div class={classes!("cactpot-grid")}> 
                { (0..NUM_CELLS).map(|i| {
                    let numbers = numbers.clone();
                    let on_wheel = on_wheel.clone();
                    let on_clear = on_clear.clone();
                    let value = numbers[i];
                    let onwheel = Callback::from(move |e: web_sys::WheelEvent| {
                        e.prevent_default();
                        let delta = e.delta_y() as i32;
                        on_wheel.emit((i, delta));
                    });
                    let oncontextmenu = Callback::from(move |e: web_sys::MouseEvent| {
                        e.prevent_default();
                        on_clear.emit(i);
                    });
                    html! {
                        <div class={classes!("cactpot-cell")}
                             tabindex="0"
                             onwheel={onwheel}
                             oncontextmenu={oncontextmenu}>
                            { value.map(|n| n.to_string()).unwrap_or_default() }
                        </div>
                    }
                }).collect::<Html>() }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
