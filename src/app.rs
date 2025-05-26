use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App(initial_length: usize) -> impl IntoView {
    let mut next_counter_id = initial_length;
    let initial_counters = (0..initial_length)
        .map(|id| (id, ArcRwSignal::new(id + 1)))
        .collect::<Vec<_>>();
    let (counters, set_counters) = signal(initial_counters);
    let add_counter = move |_| {
        let sig = ArcRwSignal::new(next_counter_id + 1);
        set_counters.update(move |counters| {
            counters.push((next_counter_id, sig))
        });
        next_counter_id += 1;
    };

    view! {
        <h2 class="title">Killer Sudoku Combo</h2>
        <div class="main-app">
            <button on:click=add_counter class="btn add-block">"Add block"</button>
            <div class="blocks">
                <For each=move || counters.get()
                    key=|counter| counter.0
                    children=move |(id, _count)| {
                        view! {
                            <KscBlock />
                            <button class="btn remove" on:click=move |_| {
                                set_counters
                                    .write()
                                    .retain(|(counter_id, _)| { counter_id != &id });
                            }>"remove"</button>
                        }
                    }
                />
            </div>
        </div>
    }

}

#[component]
pub fn KscBlock() -> impl IntoView {
    let (snum, set_snum) = signal::<usize>(0);
    let (ssum, set_ssum) = signal::<usize>(0);
    let (with, set_with) = signal::<Vec<usize>>(Vec::new());
    let (witho, set_witho) = signal::<Vec<usize>>(Vec::new());
    let num: [usize; 9] = [1,2,3,4,5,6,7,8,9];
    let sum: [usize; 45] = core::array::from_fn(|i| i + 1);
    view! {
        <div class="block">
            <div class="block-select">
                <select name="sd" class="form-select"
                    on:change:target=move |ev| {set_snum.set(ev.target().value().parse::<usize>().unwrap())}
                    prop:value=move || snum.get().to_string()
                >
                    <option value=0 selected>num</option>
                    {num.into_iter()
                        .map(|n| view! { <option value={n}>{n}</option>})
                        .collect_view()}

                </select>
                <select name="ss" class="form-select"
                    on:change:target=move |ev| {set_ssum.set(ev.target().value().parse::<usize>().unwrap())}
                    prop:value=move || ssum.get().to_string()
                >
                    <option value=0 selected>sum</option>
                    {sum.into_iter()
                        .map(|n| view! { <option value={n}>{n}</option>})
                        .collect_view()}
                </select>
                <button class="btn reset" on:click=move |_|
                    {set_snum.set(0);
                    set_ssum.set(0); 
                    set_with.set(Vec::new());
                    set_witho.set(Vec::new())}
                >reset</button>
            </div>
            <div class="inin">
                <small inner_html={"&isin;"}></small>
                {num.into_iter()
                        .map(|i| view! { 
                            <div>
                                <label class="form-check-label"><span>{i}</span>
                                    <input class="form-check-input" name="digit" type="checkbox" value={i}
                                        on:change:target=move |ev| {
                                            let mut t  = with.get();
                                            t.sort();
                                            if let Ok(a) = t.binary_search(&i){
                                                t.remove(a);
                                                set_with.set(t);
                                            }else if let Err(_e) = t.binary_search(&i){
                                                set_with.write().push(ev.target().value().parse::<usize>().unwrap());
                                            }
                                        }
                                        prop:checked={move || with.get().contains(&i)}
                                    />
                                </label>
                            </div>
                        })
                        .collect::<Vec<_>>()}

            </div>
            <div class="notin">
                <small inner_html={"&notin;"}></small> 
                {num.into_iter()
                        .map(|i| view! {
                            <div>
                                <label class="form-check-label">
                                    <input class="form-check-input" name="digit" type="checkbox" value={i}
                                        checked=move || {witho.get().binary_search(&i).is_ok()}
                                        on:change:target=move |ev| {
                                            let mut t  = witho.get();
                                            t.sort();
                                            if let Ok(a) = t.binary_search(&i){
                                                t.remove(a);
                                                set_witho.set(t);
                                                ev.target().set_checked(false);
                                            }else if let Err(_e) = t.binary_search(&i){
                                                set_witho.write().push(ev.target().value().parse::<usize>().unwrap());
                                            }
                                        }
                                        prop:checked={move || witho.get().contains(&i)}
                                    />
                                </label>
                            </div>
                        })
                        .collect_view()}

            </div>
            <div class="block-result">
                <Show
                    when=move || { snum.get() > 0 && ssum.get() > 0}
                    fallback=|| view! { <p>select number and sum</p> }
                >
                {let result = get_result(snum.get(),ssum.get());
                if let Err(e) = result{
                    view! {<p class="text-danger">{e}</p>}.into_any()
                }else if let Ok(mut ar) = result{
                    ar.sort();
                    view! {
                        <div class="n-result">
                            {ar.into_iter().map(|c| view!{
                                <span class=get_class(c.clone(),with.get(),witho.get())>{c.clone()}</span>
                            }).collect_view()}
                        </div>
                    }.into_any()
                } else {panic!("panic!!")}.into_any()
                }
                </Show>
            </div>
        </div>
    }
}



fn get_class(v:Vec<usize>,w:Vec<usize>,wo:Vec<usize>)->String{
    let mut string_class:String = String::new();

    let vh: HashSet<_> = v.iter().collect();
    let wh: HashSet<_> = w.iter().collect();
    let woh: HashSet<_> = wo.iter().collect();
    let intersect: HashSet<_> = wh.intersection(&woh).collect();
    if intersect.len() > 0{
        string_class.push_str("error_intersect");
        return string_class;
    }else{
        if wo.len()>0 {
            let intersect_wo: HashSet<_> = vh.intersection(&woh).collect();
            if intersect_wo.len() > 0{
                string_class.push_str(" naf");
            }
        }
        if w.len()>0 {
            let intersect_w: HashSet<_> = vh.intersection(&wh).collect();
            if intersect_w.len() < wh.len(){
                string_class.push_str(" naf");
            }
        }
    }
    string_class
}

fn get_result(n: usize, s: usize) -> Result<Vec<Vec<usize>>, String> {
    let gac = get_all_combi();
    if let Some(sum_map) = gac.get(&n) {
        if let Some(combinations) = sum_map.get(&s) {
            return Ok(combinations.clone());
        }
    }
    Err(String::from("this combination has never existed"))
}

fn get_all_combi() -> HashMap<usize, HashMap<usize, Vec<Vec<usize>>>> {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let num = numbers.len();
    let total = 1 << num; // Total combinations
    let mut combi: HashMap<usize, HashMap<usize, Vec<Vec<usize>>>> = HashMap::new();

    for i in 0..total {
        let ar: Vec<usize> = (0..num)
            .filter_map(|j| if (1 << j) & i != 0 { Some(numbers[j]) } else { None })
            .collect();
        
        if !ar.is_empty() {
            let sum: usize = ar.iter().sum();
            let length = ar.len();
            combi.entry(length).or_default().entry(sum).or_default().push(ar);
        }
    }
    combi
}

