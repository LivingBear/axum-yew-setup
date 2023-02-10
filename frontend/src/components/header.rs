use yew::prelude::*;

#[function_component]
pub fn HeaderComponent() -> Html {
    html! {
        <div class="header">
        <nav class="nav">
            <div class="nav-logo">
                <a href="/">
                <svg width="24" height="24" viewBox="0 0 24 24">
                    <path d="M18.6,6.62C17.16,6.62 15.8,7.18 14.83,8.15L7.8,14.39C7.16,15.03 6.31,15.38 5.4,15.38C3.53,15.38 2,13.87 2,12C2,10.13 3.53,8.62 5.4,8.62C6.31,8.62 7.16,8.97 7.84,9.65L8.97,10.65L10.5,9.31L9.22,8.2C8.2,7.18 6.84,6.62 5.4,6.62C2.42,6.62 0,9.04 0,12C0,14.96 2.42,17.38 5.4,17.38C6.84,17.38 8.2,16.82 9.17,15.85L16.2,9.61C16.84,8.97 17.69,8.62 18.6,8.62C20.47,8.62 22,10.13 22,12C22,13.87 20.47,15.38 18.6,15.38C17.7,15.38 16.84,15.03 16.16,14.35L15,13.34L13.5,14.68L14.78,15.8C15.8,16.81 17.15,17.37 18.6,17.37C21.58,17.37 24,14.96 24,12C24,9 21.58,6.62 18.6,6.62Z" /> </svg>
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
