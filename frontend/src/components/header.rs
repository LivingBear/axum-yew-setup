use yew::prelude::*;

#[function_component]
pub fn HeaderComponent() -> Html {
    html! {
        <div class="header">
        <nav class="nav">
            <div class="nav-logo">
                <a href="/">
                // <img class={classes!("h-12", "w-auto")} src="./public/img/dragon4.jpg" alt="DRAGON"/>
                <img class={classes!("h-12", "w-auto")} src="./public/img/logo1_high_res.png" alt="Living Bear"/>
                </a>
            </div>
            <div class="nav-center">
                <ul class="nav-links">
                    <div class="nav-link">
                        <a href="/about">{"About"}</a>
                    </div>
                    <div class="nav-dropdown">
                        <div class="dropdown-on-hover">{"Entertainment"}</div>
                        <ul class="dropdown-links">
                            <li class="dropdown-link">
                                <a href="/entertainment/videos">{"Videos"}</a>
                            </li>
                            <li class="dropdown-link">
                                <a href="/entertainment/books">{"Books"}</a>
                            </li>
                            <li class="dropdown-link">
                                <a href="/entertainment/blogs">{"Blogs"}</a>
                            </li>
                            <li class="dropdown-link">
                                <a href="/entertainment/images">{"Images"}</a>
                            </li>
                        </ul>
                    </div>
                    <div class="nav-dropdown">
                        <div class="dropdown-on-hover">{"Products"}</div>
                        <ul class="dropdown-links">
                            <li class="dropdown-link">
                                <a href="/products/books">{"Books"}</a>
                            </li>
                            <li class="dropdown-link">
                                <a href="/products/software">{"Software"}</a>
                            </li>
                            <li class="dropdown-link">
                                <a href="/products/courses">{"Courses"}</a>
                            </li>
                        </ul>
                    </div>
                    <div class="nav-dropdown">
                        <div class="dropdown-on-hover">{"Services"}</div>
                        <ul class="dropdown-links">
                            <li class="dropdown-link">
                                <a href="/services/personal-training">{"Personal Training"}</a>
                            </li>
                            <li class="dropdown-link">
                                <a href="/services/mentoring">{"Mentoring"}</a>
                            </li>
                            <li class="dropdown-link">
                                <a href="/services/software">{"Software as a Service"}</a>
                            </li>
                        </ul>
                    </div>
                    <div class="nav-dropdown">
                        <div class="dropdown-on-hover">{"API"}</div>
                        <ul class="dropdown-links">
                            <li class="dropdown-link">
                                <a href="/get_email_list">{"Email Data"}</a>
                            </li>
                            <li class="dropdown-link">
                                <a href="/hi-server">{"Hi Server"}</a>
                            </li>
                            <li class="dropdown-link">
                                <a href="/users">{"Users"}</a>
                            </li>
                        </ul>
                    </div>
                </ul>
            </div>
            <div class="nav-auth">
                <div class="nav-dropdown">
                <div class="dropdown-on-hover">{"Account"}</div>
                    <ul class="dropdown-links">
                        <li class="dropdown-link">
                            <a href="/profile">{"Profile"}</a>

                        </li>
                        <li class="dropdown-link">
                            <a href="/admin">{"Admin"}</a>

                        </li>
                        <li class="dropdown-link">
                            <a href="/email">{"Email"}</a>

                        </li>
                        <li class="dropdown-link">
                            <a href="/login">{"Login"}</a>

                        </li>
                        <li class="dropdown-link">
                            <a href="/register">{"Register"}</a>

                        </li>
                    </ul>
                </div>
            </div>
        </nav>
        </div>

    }
}
