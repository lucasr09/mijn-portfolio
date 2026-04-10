use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let projects = vec![
        (
            "Rocket Web Application",
            "A web application built with Rocket for a snackbar, focused on backend structure, routing, and performance.",
            "Rust, Rocket",
            "https://github.com/lucasr09/Website-Frame-RustRocket",
        ),
        (
            "Rust Portfolio",
            "A portfolio website built with Rust and Leptos, designed to present projects with a focus on performance, simplicity, and clean UI.",
            "Rust, Leptos, Trunk",
            "https://github.com/lucasr09/mijn-portfolio",
        ),
        (
            "2D Platformer Game",
            "A 2D platformer built in Unity, focused on gameplay systems, player movement, and level design.",
            "C#, Unity",
            "https://github.com/lucasr09/GameProject",
        ),
    ];

    let skills = vec![
        "Rust",
        "Rocket",
        "Leptos",
        "Python",
        "Django",
        "JavaScript",
        "React",
        "Next.js",
        "Unity",
        "Git",
        "Linux",
    ];

    view! {
        <div class="site-shell">
            <header class="topbar">
                <div class="container nav-inner">
                    <a class="logo" href="#home">"LR"</a>
                    <nav class="nav-links">
                        <a href="#about">"About"</a>
                        <a href="#projects">"Projects"</a>
                        <a href="#skills">"Skills"</a>
                        <a href="#contact">"Contact"</a>
                    </nav>
                </div>
            </header>

            <main>
                <section id="home" class="hero">
                    <div class="container hero-card">
                        <p class="eyebrow">"Rust • Backend • Web Development"</p>
                        <h1>"Lucas Rensen"</h1>
                        <p class="hero-copy">
                            "I build software with a focus on Rust, backend development, and clean web experiences."
                        </p>
                        <div class="hero-actions">
                            <a class="btn btn-primary" href="#projects">"View Projects"</a>
                            <a class="btn btn-secondary" href="mailto:lucasrensen@outlook.com">"Get in Touch"</a>
                        </div>
                    </div>
                </section>

                <section id="about" class="section">
                    <div class="container split">
                        <div>
                            <p class="section-label">"About"</p>
                            <h2>"Developer focused on building solid software without unnecessary complexity."</h2>
                        </div>
                        <div class="content-card">
                            <p>
                                "I enjoy building software that is practical, structured, and technically honest."
                            </p>
                            <p>
                                "My main interests are Rust, backend development, web applications, and interactive projects such as games."
                            </p>
                            <p>
                                "I care about clean code, clear architecture, and building things that are useful rather than overdesigned."
                            </p>
                        </div>
                    </div>
                </section>

                <section id="projects" class="section">
                    <div class="container">
                        <p class="section-label">"Projects"</p>
                        <h2>"Projects that reflect my technical direction"</h2>
                        <div class="grid projects-grid">
                            {projects
                                .into_iter()
                                .map(|(title, description, stack, link)| {
                                    view! {
                                        <article class="project-card">
                                            <p class="project-stack">{stack}</p>
                                            <h3>{title}</h3>
                                            <p>{description}</p>
                                            <a class="project-link" href=link target="_blank" rel="noopener noreferrer">
                                                "View on GitHub"
                                            </a>
                                        </article>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>
                </section>

                <section id="skills" class="section">
                    <div class="container">
                        <p class="section-label">"Skills"</p>
                        <h2>"Technologies I work with"</h2>
                        <div class="skills-wrap">
                            {skills
                                .into_iter()
                                .map(|skill| {
                                    view! { <span class="skill-pill">{skill}</span> }
                                })
                                .collect_view()}
                        </div>
                    </div>
                </section>

                <section id="contact" class="section">
                    <div class="container contact-card">
                        <div>
                            <p class="section-label">"Contact"</p>
                            <h2>"Let's build something solid."</h2>
                            <p class="contact-copy">
                                "For projects, collaboration, or simply a good technical conversation."
                            </p>
                        </div>
                        <div class="contact-links">
                            <a href="mailto:lucasrensen@outlook.com">"lucasrensen@outlook.com"</a>
                            <a href="https://github.com/lucasr09" target="_blank" rel="noopener noreferrer">
                                "GitHub"
                            </a>
                            <a href="https://www.linkedin.com/in/lucas-rensen-b18b40232/" target="_blank" rel="noopener noreferrer">
                                "LinkedIn"
                            </a>
                        </div>
                    </div>
                </section>
            </main>
        </div>
    }
}