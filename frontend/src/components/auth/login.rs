use yew::prelude::*;

#[function_component(LoginComponent)]
pub fn login() -> Html {
    html! {       
        <div class={classes!("box")}>
            <div class={classes!("container")}>
                <div class={classes!("top")}>
                    <span>{"Have an account?"}</span>
                    <header>{"Login"}</header>
                </div>
                <div class={classes!("input-field")}>
                    <input type="text" class={classes!("input")} placeholder="Username" id=""/>
                    <i class={classes!("bx", "bx-user")}></i>
                </div>
                <div class={classes!("input-field")}>
                    <input type="Password" class={classes!("input")} placeholder="Password" id=""/>
                    <i class={classes!("bx", "bx-lock-alt")}></i>
                </div>
                <div class={classes!("input-field")}>
                    <input type="submit" class={classes!("submit")} value="Login" id=""/>
                </div>
                <div class={classes!("two-col")}>
                    <div class={classes!("one")}>
                    <input type="checkbox" name="" id="check"/>
                    <label for="check"> {"Remember Me"}</label>
                    </div>
                    <div class={classes!("two")}>
                        <label><a href="/forgot-password">{"Forgot password?"}</a></label>
                    </div>
                </div>
            </div>
        </div>
    }
}