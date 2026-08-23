use leptos::html;
use leptos::prelude::*;
use leptos_use::{use_color_mode, use_intersection_observer, use_window_scroll, ColorMode, UseColorModeReturn};

#[component]
pub fn App() -> impl IntoView {
    let projects = vec![
        (
            "Kolkie Website",
            "A web application built with Rocket for a snackbar, focused on backend structure, routing, and performance.",
            "Rust, Rocket",
            "https://github.com/lucasr09/KolkieWebsite",
        ),
        (
            "EU5 Mission Mod",
            "Four full mission trees (Denmark, Norway, Iceland, Ireland) contributed to a community mod for Europa Universalis V, written in Paradox's scripting format.",
            "Paradox Script, EU5 Modding",
            "https://github.com/lucasr09/Eu5MissionMod",
        ),
        (
            "Kolkie Inkloksysteem",
            "A clock-in and scheduling system for the staff of Cafetaria Kolkie, with a Rust backend and a reactive Svelte frontend backed by SQLite.",
            "Rust, Rocket, Svelte, SQLite",
            "https://github.com/lucasr09/kolkie-inklok",
        ),
    ];

    let skills = vec![
        "Rust",
        "Rocket",
        "Leptos",
        "C#",
        "Python",
        "JavaScript",
        "React",
        "Next.js",
        "PHP",
        "Unity",
        "Git",
        "Linux",
    ];

    // De marquee bevat de lijst 2 keer: de CSS-animatie schuift precies -50%
    // en sluit dan naadloos aan op de tweede set (zelfde truc als de fotoslider
    // op de Kolkie-website, maar dan voor tekst-pills).
    let skills_marquee: Vec<&str> = skills.iter().chain(skills.iter()).copied().collect();

    // Elke sectie krijgt een eigen IntersectionObserver: "revealed" wordt één keer
    // true (voor de scroll-in animatie), "in_view" volgt de sectie heen en weer
    // (voor de actieve link in de nav, scrollspy-stijl).
    let about_ref = NodeRef::<html::Section>::new();
    let projects_ref = NodeRef::<html::Section>::new();
    let skills_ref = NodeRef::<html::Section>::new();
    let contact_ref = NodeRef::<html::Section>::new();

    let (about_revealed, set_about_revealed) = signal(false);
    let (about_in_view, set_about_in_view) = signal(false);
    let (projects_revealed, set_projects_revealed) = signal(false);
    let (projects_in_view, set_projects_in_view) = signal(false);
    let (skills_revealed, set_skills_revealed) = signal(false);
    let (skills_in_view, set_skills_in_view) = signal(false);
    let (contact_revealed, set_contact_revealed) = signal(false);
    let (contact_in_view, set_contact_in_view) = signal(false);

    use_intersection_observer(about_ref, move |entries, _| {
        if let Some(entry) = entries.first() {
            let intersecting = entry.is_intersecting();
            set_about_in_view.set(intersecting);
            if intersecting {
                set_about_revealed.set(true);
            }
        }
    });
    use_intersection_observer(projects_ref, move |entries, _| {
        if let Some(entry) = entries.first() {
            let intersecting = entry.is_intersecting();
            set_projects_in_view.set(intersecting);
            if intersecting {
                set_projects_revealed.set(true);
            }
        }
    });
    use_intersection_observer(skills_ref, move |entries, _| {
        if let Some(entry) = entries.first() {
            let intersecting = entry.is_intersecting();
            set_skills_in_view.set(intersecting);
            if intersecting {
                set_skills_revealed.set(true);
            }
        }
    });
    use_intersection_observer(contact_ref, move |entries, _| {
        if let Some(entry) = entries.first() {
            let intersecting = entry.is_intersecting();
            set_contact_in_view.set(intersecting);
            if intersecting {
                set_contact_revealed.set(true);
            }
        }
    });

    let (_scroll_x, scroll_y) = use_window_scroll();
    let nav_scrolled = move || scroll_y.get() > 8.0;

    let (nav_open, set_nav_open) = signal(false);
    let close_nav = move || set_nav_open.set(false);

    let UseColorModeReturn { mode, set_mode, .. } = use_color_mode();
    let is_dark = move || mode.get() == ColorMode::Dark;
    let toggle_theme = move |_| {
        if is_dark() {
            set_mode.set(ColorMode::Light);
        } else {
            set_mode.set(ColorMode::Dark);
        }
    };

    // use_color_mode() past de "dark"-class op <html> zelf niet betrouwbaar
    // opnieuw toe na een reload (de voorkeur staat wel goed in localStorage,
    // de class-sync bleek het niet te doen) — dus expliciet zelf bijhouden.
    Effect::new(move |_| {
        if let Some(el) = document().document_element() {
            let _ = if is_dark() {
                el.class_list().add_1("dark")
            } else {
                el.class_list().remove_1("dark")
            };
        }
    });

    view! {
        <div class="site-shell">
            <header class="topbar" class:scrolled=nav_scrolled>
                <div class="container nav-inner">
                    <a class="logo" href="#home">"LR"</a>

                    <div class="nav-right">
                        <button class="theme-toggle" aria-label="Wissel tussen licht en donker thema" on:click=toggle_theme>
                            <Show
                                when=is_dark
                                fallback=|| view! {
                                    // Maan-icoon: klik om naar donker te wisselen.
                                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79Z" />
                                    </svg>
                                }
                            >
                                // Zon-icoon: klik om naar licht te wisselen.
                                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <circle cx="12" cy="12" r="4" />
                                    <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41" />
                                </svg>
                            </Show>
                        </button>

                        <button
                            class="nav-toggle"
                            class:open=move || nav_open.get()
                            aria-label="Menu"
                            aria-expanded=move || nav_open.get().to_string()
                            on:click=move |_| set_nav_open.update(|open| *open = !*open)
                        >
                            <span></span>
                            <span></span>
                            <span></span>
                        </button>

                        <nav class="nav-links" class:open=move || nav_open.get()>
                            <a href="#about" class:active=move || about_in_view.get() on:click=move |_| close_nav()>"About"</a>
                            <a href="#projects" class:active=move || projects_in_view.get() on:click=move |_| close_nav()>"Projects"</a>
                            <a href="#skills" class:active=move || skills_in_view.get() on:click=move |_| close_nav()>"Skills"</a>
                            <a href="#contact" class:active=move || contact_in_view.get() on:click=move |_| close_nav()>"Contact"</a>
                        </nav>
                    </div>
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
                            <a class="btn btn-primary" href="/CV_Lucas_Rensen.pdf" target="_blank" rel="noopener noreferrer">"Resume"</a>
                            <a class="btn btn-secondary" href="mailto:lucasrensen@outlook.com">"Get in Touch"</a>
                        </div>
                        <p class="hero-location">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z" />
                                <circle cx="12" cy="10" r="3" />
                            </svg>
                            "Gelderland, NL — open to opportunities nearby"
                        </p>
                    </div>
                </section>

                <section id="about" class="section" class:revealed=move || about_revealed.get() node_ref=about_ref>
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

                <section id="projects" class="section" class:revealed=move || projects_revealed.get() node_ref=projects_ref>
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

                <section id="skills" class="section" class:revealed=move || skills_revealed.get() node_ref=skills_ref>
                    <div class="container">
                        <p class="section-label">"Skills"</p>
                        <h2>"Technologies I work with"</h2>
                        <div class="skills-marquee">
                            <div class="skills-track">
                                {skills_marquee
                                    .into_iter()
                                    .map(|skill| {
                                        view! { <span class="skill-pill">{skill}</span> }
                                    })
                                    .collect_view()}
                            </div>
                        </div>
                    </div>
                </section>

                <section id="contact" class="section" class:revealed=move || contact_revealed.get() node_ref=contact_ref>
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