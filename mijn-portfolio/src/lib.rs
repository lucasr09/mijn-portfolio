use leptos::html;
use leptos::prelude::*;
use leptos_use::{use_color_mode, use_intersection_observer, use_window_scroll, ColorMode, UseColorModeReturn};

#[derive(Clone, Copy, PartialEq)]
enum Lang {
    En,
    Nl,
}

/// Kiest de Engelse of Nederlandse variant van een stuk tekst.
fn t(lang: Lang, en: &'static str, nl: &'static str) -> &'static str {
    match lang {
        Lang::En => en,
        Lang::Nl => nl,
    }
}

fn stored_lang() -> Lang {
    window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item("lang").ok().flatten())
        .filter(|v| v == "nl")
        .map(|_| Lang::Nl)
        .unwrap_or(Lang::En)
}

struct ProjectEntry {
    title: &'static str,
    description: (&'static str, &'static str),
    stack: &'static str,
    link: &'static str,
}

struct ExperienceEntry {
    role: (&'static str, &'static str),
    company: &'static str,
    dates: (&'static str, &'static str),
    bullets: Vec<(&'static str, &'static str)>,
}

#[component]
pub fn App() -> impl IntoView {
    let projects = vec![
        ProjectEntry {
            title: "Kolkie Website",
            description: (
                "A web application built with Rocket for a snackbar, focused on backend structure, routing, and performance.",
                "Een webapplicatie gebouwd met Rocket voor een snackbar, gericht op backend-structuur, routing en performance.",
            ),
            stack: "Rust, Rocket",
            link: "https://github.com/lucasr09/KolkieWebsite",
        },
        ProjectEntry {
            title: "EU5 Mission Mod",
            description: (
                "Four full mission trees (Denmark, Norway, Iceland, Ireland) contributed to a community mod for Europa Universalis V, written in Paradox's scripting format.",
                "Vier volledige missiebomen (Denemarken, Noorwegen, IJsland, Ierland) toegevoegd aan een community-mod voor Europa Universalis V, geschreven in de scripttaal van Paradox.",
            ),
            stack: "Paradox Script, EU5 Modding",
            link: "https://github.com/lucasr09/Eu5MissionMod",
        },
        ProjectEntry {
            title: "Kolkie Inkloksysteem",
            description: (
                "A clock-in and scheduling system for the staff of Cafetaria Kolkie, with a Rust backend and a reactive Svelte frontend backed by SQLite.",
                "Een in- en uitkloksysteem met roosterfunctie voor het personeel van Cafetaria Kolkie, met een Rust-backend en een reactieve Svelte-frontend op SQLite.",
            ),
            stack: "Rust, Rocket, Svelte, SQLite",
            link: "https://github.com/lucasr09/kolkie-inklok",
        },
    ];

    // Eigen sectie onder "Projects", zelfde kaart-opmaak: werk dat bewust als
    // prototype / proof of concept blijft staan, niet als afgerond project.
    let prototypes = vec![
        ProjectEntry {
            title: "Broodjeszaak bon-generator",
            description: (
                "At my job at Ardoer Camping De Jutberg I kept retyping paid bread orders from the campsite website into the register by hand to print a kitchen slip, and saw it could be much simpler. So I built this: check an order once, print the slip straight away, and never accidentally print the same order twice. Python CLI and a Flask web app.",
                "Op mijn werk bij Ardoer Camping De Jutberg zat ik telkens al-betaalde broodjesbestellingen van de campingsite met de hand over te typen op de kassa om er een keukenbon van te printen, en ik zag dat het veel simpeler kon. Dus bouwde ik dit: een bestelling \u{00e9}\u{00e9}n keer controleren, meteen de bon printen, en nooit per ongeluk dezelfde bestelling dubbel de keuken in. Python-CLI en een Flask-webapp.",
            ),
            stack: "Python, Flask",
            link: "https://github.com/lucasr09/broodjes_zaak_con",
        },
    ];

    let experience = vec![
        ExperienceEntry {
            role: ("Software Developer — Internship", "Software Developer — Stage"),
            company: "ProRail",
            dates: ("Feb 2026 – Jun 2026", "feb 2026 – jun 2026"),
            bullets: vec![
                (
                    "Contributed to the development of the BTD planner",
                    "Bijgedragen aan de ontwikkeling van de BTD-planner",
                ),
                (
                    "Improved usability and system performance",
                    "Gebruiksvriendelijkheid en systeemprestaties verbeterd",
                ),
                (
                    "Worked in a Scrum team, taking part in daily standups, retrospectives, week-start planning, and BCB meetings",
                    "Gewerkt in een Scrum-team, met deelname aan daily standups, retrospectives, weekstarts en BCB-overleggen",
                ),
            ],
        },
        ExperienceEntry {
            role: ("Cafetaria Staff", "Cafetariamedewerker"),
            company: "Cafetaria Lunchroom Kolkie",
            dates: ("Nov 2025 – Mar 2026", "nov 2025 – mrt 2026"),
            bullets: vec![(
                "Worked the counter and kitchen at the business behind my Kolkie Website and Kolkie Inkloksysteem projects",
                "Achter de balie en in de keuken gewerkt bij de zaak waarvoor ik ook de Kolkie Website en het Kolkie Inkloksysteem heb gebouwd",
            )],
        },
        ExperienceEntry {
            role: ("Software Developer — Internship", "Software Developer — Stage"),
            company: "Coldenhove Papierfabriek",
            dates: ("Feb 2025 – Jul 2025", "feb 2025 – jul 2025"),
            bullets: vec![
                (
                    "Built a new company-wide phone directory application",
                    "Nieuwe bedrijfsbrede telefoonlijst-applicatie gebouwd",
                ),
                (
                    "Developed a central launcher tool built around one generic \"open file\" function, so staff could jump to frequently used programs with a click instead of navigating to them manually",
                    "Centrale launcher-tool ontwikkeld rond één generieke \"open file\"-functie, zodat collega's met één klik naar veelgebruikte programma's konden springen in plaats van er handmatig naartoe te navigeren",
                ),
                (
                    "Assisted with automation projects",
                    "Meegelopen met automatiseringsprojecten",
                ),
            ],
        },
        ExperienceEntry {
            role: ("Software Developer — Internship", "Software Developer — Stage"),
            company: "ACTwebservice",
            dates: ("Sep 2023 – Feb 2024", "sep 2023 – feb 2024"),
            bullets: vec![
                (
                    "Built websites for external clients using Duda",
                    "Websites gebouwd voor externe klanten met Duda",
                ),
                ("Handled client conversations", "Klantgesprekken gevoerd"),
                (
                    "Designed and set up databases",
                    "Databases ontworpen en opgezet",
                ),
            ],
        },
    ];

    let skills = vec![
        "Rust",
        "Rocket",
        "Leptos",
        "C#",
        "Python",
        "Flask",
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
    // op de Kolkie-website, maar dan voor tekst-pills). Taal-onafhankelijk,
    // want technologienamen worden niet vertaald.
    let skills_marquee: Vec<&str> = skills.iter().chain(skills.iter()).copied().collect();

    let (lang, set_lang) = signal(stored_lang());
    let toggle_lang = move |_| {
        set_lang.update(|l| *l = if *l == Lang::En { Lang::Nl } else { Lang::En });
    };

    // Voorkeur onthouden in localStorage, en de "lang"-attribuut op <html>
    // meegeven (belangrijk voor screenreaders en correcte uitspraak).
    Effect::new(move |_| {
        let code = if lang.get() == Lang::Nl { "nl" } else { "en" };
        if let Ok(Some(storage)) = window().local_storage() {
            let _ = storage.set_item("lang", code);
        }
        if let Some(el) = document().document_element() {
            let _ = el.set_attribute("lang", code);
        }
    });

    // Elke sectie krijgt een eigen IntersectionObserver: "revealed" wordt één keer
    // true (voor de scroll-in animatie), "in_view" volgt de sectie heen en weer
    // (voor de actieve link in de nav, scrollspy-stijl).
    let about_ref = NodeRef::<html::Section>::new();
    let experience_ref = NodeRef::<html::Section>::new();
    let projects_ref = NodeRef::<html::Section>::new();
    let prototype_ref = NodeRef::<html::Section>::new();
    let skills_ref = NodeRef::<html::Section>::new();
    let contact_ref = NodeRef::<html::Section>::new();

    let (about_revealed, set_about_revealed) = signal(false);
    let (about_in_view, set_about_in_view) = signal(false);
    let (experience_revealed, set_experience_revealed) = signal(false);
    let (experience_in_view, set_experience_in_view) = signal(false);
    let (projects_revealed, set_projects_revealed) = signal(false);
    let (projects_in_view, set_projects_in_view) = signal(false);
    let (prototype_revealed, set_prototype_revealed) = signal(false);
    let (prototype_in_view, set_prototype_in_view) = signal(false);
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
    use_intersection_observer(experience_ref, move |entries, _| {
        if let Some(entry) = entries.first() {
            let intersecting = entry.is_intersecting();
            set_experience_in_view.set(intersecting);
            if intersecting {
                set_experience_revealed.set(true);
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
    use_intersection_observer(prototype_ref, move |entries, _| {
        if let Some(entry) = entries.first() {
            let intersecting = entry.is_intersecting();
            set_prototype_in_view.set(intersecting);
            if intersecting {
                set_prototype_revealed.set(true);
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
                        <button
                            class="lang-toggle"
                            aria-label=move || t(lang.get(), "Switch to Dutch", "Overschakelen naar Engels")
                            on:click=toggle_lang
                        >
                            {move || t(lang.get(), "NL", "EN")}
                        </button>

                        <button
                            class="theme-toggle"
                            aria-label=move || t(lang.get(), "Toggle theme", "Wissel van thema")
                            on:click=toggle_theme
                        >
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
                            <a href="#about" class:active=move || about_in_view.get() on:click=move |_| close_nav()>{move || t(lang.get(), "About", "Over mij")}</a>
                            <a href="#experience" class:active=move || experience_in_view.get() on:click=move |_| close_nav()>{move || t(lang.get(), "Experience", "Ervaring")}</a>
                            <a href="#projects" class:active=move || projects_in_view.get() on:click=move |_| close_nav()>{move || t(lang.get(), "Projects", "Projecten")}</a>
                            <a href="#prototype" class:active=move || prototype_in_view.get() on:click=move |_| close_nav()>{move || t(lang.get(), "Prototypes", "Prototypes")}</a>
                            <a href="#skills" class:active=move || skills_in_view.get() on:click=move |_| close_nav()>{move || t(lang.get(), "Skills", "Vaardigheden")}</a>
                            <a href="#contact" class:active=move || contact_in_view.get() on:click=move |_| close_nav()>{move || t(lang.get(), "Contact", "Contact")}</a>
                        </nav>
                    </div>
                </div>
            </header>

            <main>
                <section id="home" class="hero">
                    <div class="container hero-card">
                        <p class="eyebrow">{move || t(lang.get(), "Rust • Backend • Web Development", "Rust • Backend • Webontwikkeling")}</p>
                        <h1>"Lucas Rensen"</h1>
                        <p class="hero-copy">
                            {move || t(
                                lang.get(),
                                "I build software with a focus on Rust, backend development, and clean web experiences.",
                                "Ik bouw software met een focus op Rust, backend-ontwikkeling en overzichtelijke webapplicaties.",
                            )}
                        </p>
                        <div class="hero-actions">
                            <a class="btn btn-primary" href="/CV_Lucas_Rensen.pdf" target="_blank" rel="noopener noreferrer">
                                {move || t(lang.get(), "Resume", "CV")}
                            </a>
                            <a class="btn btn-secondary" href="mailto:lucasrensen@outlook.com">
                                {move || t(lang.get(), "Get in Touch", "Neem contact op")}
                            </a>
                        </div>
                        <p class="hero-location">
                            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <path d="M20 10c0 6-8 12-8 12s-8-6-8-12a8 8 0 0 1 16 0Z" />
                                <circle cx="12" cy="10" r="3" />
                            </svg>
                            {move || t(lang.get(), "Gelderland, NL — open to opportunities nearby", "Gelderland, NL — open voor kansen in de regio")}
                        </p>
                    </div>
                </section>

                <section id="about" class="section" class:revealed=move || about_revealed.get() node_ref=about_ref>
                    <div class="container split">
                        <div>
                            <p class="section-label">{move || t(lang.get(), "About", "Over mij")}</p>
                            <h2>{move || t(
                                lang.get(),
                                "Developer focused on building solid software without unnecessary complexity.",
                                "Developer die solide software bouwt, zonder onnodige complexiteit.",
                            )}</h2>
                        </div>
                        <div class="content-card">
                            <p>
                                {move || t(
                                    lang.get(),
                                    "I enjoy building software that is practical, structured, and technically honest.",
                                    "Ik bouw graag software die praktisch, gestructureerd en zonder franje in elkaar zit.",
                                )}
                            </p>
                            <p>
                                {move || t(
                                    lang.get(),
                                    "My main interests are Rust, backend development, web applications, and interactive projects such as games.",
                                    "Mijn belangrijkste interesses zijn Rust, backend-ontwikkeling, webapplicaties en interactieve projecten zoals games.",
                                )}
                            </p>
                            <p>
                                {move || t(
                                    lang.get(),
                                    "I care about clean code, clear architecture, and building things that are useful rather than overdesigned.",
                                    "Ik hecht waarde aan schone code, heldere architectuur en het bouwen van dingen die nuttig zijn in plaats van onnodig ingewikkeld.",
                                )}
                            </p>
                        </div>
                    </div>
                </section>

                <section id="experience" class="section" class:revealed=move || experience_revealed.get() node_ref=experience_ref>
                    <div class="container">
                        <p class="section-label">{move || t(lang.get(), "Experience", "Ervaring")}</p>
                        <h2>{move || t(
                            lang.get(),
                            "Hands-on experience from four internships and steady work alongside my studies",
                            "Praktijkervaring uit vier stages en vast werk naast mijn studie",
                        )}</h2>
                        <div class="timeline">
                            {move || experience
                                .iter()
                                .map(|entry| {
                                    let role = t(lang.get(), entry.role.0, entry.role.1);
                                    let dates = t(lang.get(), entry.dates.0, entry.dates.1);
                                    let bullets = entry.bullets.clone();
                                    view! {
                                        <div class="timeline-item">
                                            <div class="timeline-marker"></div>
                                            <div class="timeline-content">
                                                <p class="timeline-dates">{dates}</p>
                                                <h3>{role}</h3>
                                                <p class="timeline-company">{entry.company}</p>
                                                <ul>
                                                    {bullets.into_iter().map(|(en, nl)| view! { <li>{t(lang.get(), en, nl)}</li> }).collect_view()}
                                                </ul>
                                            </div>
                                        </div>
                                    }
                                })
                                .collect_view()}
                        </div>
                        <p class="timeline-note">
                            {move || t(
                                lang.get(),
                                "Also worked consistently in hospitality alongside school: kitchen and service staff at Ardoer Camping De Jutberg (seasonal, 2023–present) and lifeguard duty there in the summer of 2021.",
                                "Daarnaast al jaren horecawerk naast school: keuken en bediening bij Ardoer Camping De Jutberg (seizoenswerk, 2023–heden), en badmeester in de zomer van 2021.",
                            )}
                        </p>
                    </div>
                </section>

                <section id="projects" class="section" class:revealed=move || projects_revealed.get() node_ref=projects_ref>
                    <div class="container">
                        <p class="section-label">{move || t(lang.get(), "Projects", "Projecten")}</p>
                        <h2>{move || t(
                            lang.get(),
                            "Projects that reflect my technical direction",
                            "Projecten die mijn technische richting laten zien",
                        )}</h2>
                        <div class="grid projects-grid">
                            {move || projects
                                .iter()
                                .map(|p| {
                                    let description = t(lang.get(), p.description.0, p.description.1);
                                    view! {
                                        <article class="project-card">
                                            <p class="project-stack">{p.stack}</p>
                                            <h3>{p.title}</h3>
                                            <p>{description}</p>
                                            <a class="project-link" href=p.link target="_blank" rel="noopener noreferrer">
                                                {move || t(lang.get(), "View on GitHub", "Bekijk op GitHub")}
                                            </a>
                                        </article>
                                    }
                                })
                                .collect_view()}
                        </div>
                    </div>
                </section>

                <section id="prototype" class="section" class:revealed=move || prototype_revealed.get() node_ref=prototype_ref>
                    <div class="container">
                        <p class="section-label">{move || t(lang.get(), "Prototypes", "Prototypes")}</p>
                        <h2>{move || t(
                            lang.get(),
                            "Prototypes and proof-of-concept work",
                            "Prototypes en proof-of-concept werk",
                        )}</h2>
                        <div class="grid projects-grid">
                            {move || prototypes
                                .iter()
                                .map(|p| {
                                    let description = t(lang.get(), p.description.0, p.description.1);
                                    view! {
                                        <article class="project-card">
                                            <p class="project-stack">{p.stack}</p>
                                            <h3>{p.title}</h3>
                                            <p>{description}</p>
                                            <a class="project-link" href=p.link target="_blank" rel="noopener noreferrer">
                                                {move || t(lang.get(), "View on GitHub", "Bekijk op GitHub")}
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
                        <p class="section-label">{move || t(lang.get(), "Skills", "Vaardigheden")}</p>
                        <h2>{move || t(lang.get(), "Technologies I work with", "Technologieën waar ik mee werk")}</h2>
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
                            <p class="section-label">{move || t(lang.get(), "Contact", "Contact")}</p>
                            <h2>{move || t(lang.get(), "Let's build something solid.", "Laten we iets solide bouwen.")}</h2>
                            <p class="contact-copy">
                                {move || t(
                                    lang.get(),
                                    "For projects, collaboration, or simply a good technical conversation.",
                                    "Voor projecten, samenwerking, of gewoon een goed technisch gesprek.",
                                )}
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
