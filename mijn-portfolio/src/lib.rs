use std::f64::consts::PI;
use std::time::Duration;

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

fn use_section_observer(node_ref: NodeRef<html::Section>) -> (ReadSignal<bool>, ReadSignal<bool>) {
    let (revealed, set_revealed) = signal(false);
    let (in_view, set_in_view) = signal(false);
    use_intersection_observer(node_ref, move |entries, _| {
        if let Some(entry) = entries.first() {
            let intersecting = entry.is_intersecting();
            set_in_view.set(intersecting);
            if intersecting {
                set_revealed.set(true);
            }
        }
    });
    (revealed, in_view)
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

/// Pad voor een "taartpunt-ring" (annular sector) tussen straal r1 en r2,
/// van hoek a0 tot a1 (in radialen): de bouwsteen van elk dartbordvak.
fn ring_segment(r1: f64, r2: f64, a0: f64, a1: f64) -> String {
    let p = |r: f64, a: f64| format!("{:.2} {:.2}", r * a.cos(), r * a.sin());
    format!(
        "M{} A{r2} {r2} 0 0 1 {} L{} A{r1} {r1} 0 0 0 {} Z",
        p(r2, a0),
        p(r2, a1),
        p(r1, a1),
        p(r1, a0),
    )
}

/// Decoratief dartbord in de officiële verhoudingen (in mm, straal 170 tot de
/// buitenkant van de dubbelring). Afwisselend donker/licht per vak, met rood
/// op de donkere en groen op de lichte vakken in de dubbel- en triplering,
/// zoals op een echt bord. Vak 0 staat bovenaan (de 20).
#[component]
fn Dartboard() -> impl IntoView {
    const SEG: f64 = PI / 10.0;
    // (binnenstraal, buitenstraal, is_scoring_ring)
    const BANDS: [(f64, f64, bool); 4] = [
        (15.9, 99.0, false),
        (99.0, 107.0, true),
        (107.0, 162.0, false),
        (162.0, 170.0, true),
    ];

    let segments = (0..20)
        .flat_map(|i| {
            let a0 = -PI / 2.0 + (i as f64 - 0.5) * SEG;
            let even = i % 2 == 0;
            BANDS.iter().map(move |&(r1, r2, ring)| {
                let class = match (ring, even) {
                    (true, true) => "db-red",
                    (true, false) => "db-green",
                    (false, true) => "db-dark",
                    (false, false) => "db-light",
                };
                view! { <path class=class d=ring_segment(r1, r2, a0, a0 + SEG) /> }
            })
        })
        .collect_view();

    view! {
        <svg class="dartboard" viewBox="-200 -200 400 400" aria-hidden="true">
            <circle class="db-dark" r="200" />
            {segments}
            <circle class="db-green" r="15.9" />
            <circle class="db-red" r="6.35" />
        </svg>
    }
}

struct ProjectEntry {
    title: &'static str,
    description: (&'static str, &'static str),
    stack: &'static str,
    link: &'static str,
}

struct InterestEntry {
    title: (&'static str, &'static str),
    text: (&'static str, &'static str),
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
                "A redesign of the website for Cafetaria Lunchroom Kolkie in Dieren.\n\nRust and Rocket on the backend with Askama templates, a working contact form that sends email over SMTP, and security built in from the start: rate limiting, a honeypot against bots and strict security headers. Fully responsive.",
                "Een redesign van de website van Cafetaria Lunchroom Kolkie in Dieren.\n\nRust en Rocket in de backend met Askama-templates, een werkend contactformulier dat via SMTP mailt, en beveiliging die er vanaf het begin in zit: rate limiting, een honeypot tegen bots en strikte security-headers. Volledig responsive.",
            ),
            stack: "Rust, Rocket, Askama",
            link: "https://github.com/lucasr09/KolkieWebsite",
        },
        ProjectEntry {
            title: "EU5 Mission Mod",
            description: (
                "Four full mission trees (Denmark, Norway, Iceland, Ireland) contributed to a community mod for Europa Universalis V, written in Paradox's scripting format.",
                "Vier complete missiebomen (Denemarken, Noorwegen, IJsland en Ierland) voor een community-mod van Europa Universalis V, geschreven in de scripttaal van Paradox.",
            ),
            stack: "Paradox Script, EU5 Modding",
            link: "https://github.com/lucasr09/Eu5MissionMod",
        },
        ProjectEntry {
            title: "Kolkie Inkloksysteem",
            description: (
                "A clock-in and scheduling system for the staff of Cafetaria Kolkie. Employees clock in and out, log their breaks and share their availability, while managers plan the schedule and see everyone's hours.\n\nRust and Rocket on the backend, Svelte on the frontend and SQLite as the database, secured with bcrypt-hashed passwords, rate limiting and separate roles for staff and managers.",
                "Een in- en uitkloksysteem met roosters voor het personeel van Cafetaria Kolkie. Medewerkers klokken in en uit, registreren hun pauzes en geven hun beschikbaarheid door, en managers maken het rooster en zien ieders uren.\n\nRust en Rocket in de backend, Svelte in de frontend en SQLite als database, beveiligd met gehashte wachtwoorden (bcrypt), rate limiting en aparte rollen voor medewerkers en managers.",
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
                "When I worked at Ardoer Camping De Jutberg, I kept retyping paid bread orders from the campsite website into the register by hand to print a kitchen slip, and saw it could be much simpler.\n\nSo I built this: check an order once, print the slip straight away, and never accidentally print the same order twice.\n\nA duplicate order gets flagged and can be removed with a (simulated) refund. The Python CLI and the Flask web app run on exactly the same logic.",
                "Toen ik bij Ardoer Camping De Jutberg werkte, moest ik betaalde broodjesbestellingen van de campingwebsite steeds met de hand overtypen in de kassa om een keukenbon te kunnen printen.\n\nDat kon veel simpeler, dus bouwde ik dit: je controleert een bestelling \u{00e9}\u{00e9}n keer, de bon wordt meteen geprint en dezelfde bestelling kan nooit per ongeluk twee keer naar de keuken.\n\nEen dubbel binnengekomen bestelling wordt gemarkeerd en kan met een (gesimuleerde) terugbetaling worden verwijderd. De Python-CLI en de Flask-webapp draaien op precies dezelfde logica.",
            ),
            stack: "Python, Flask",
            link: "https://github.com/lucasr09/broodjes_zaak_con",
        },
        ProjectEntry {
            title: "Wereldreiziger",
            description: (
                "I wanted an easy way to keep track of which countries I've visited, still want to visit, or where I live. Existing apps never showed the things I actually cared about: population, land area, biggest cities, and how popular a country really is with Dutch travelers.\n\nSo I built my own: a desktop app with an interactive, zoomable world map, live data from the World Bank and GeoNames, and a unique photo per country. The interface is available in six languages.\n\nIt is built entirely in Rust, with a Tauri backend and a Leptos frontend.",
                "Ik wilde makkelijk bijhouden welke landen ik heb bezocht, nog wil bezoeken en waar ik woon. Bestaande apps lieten nooit zien wat ik belangrijk vond: inwonertal, oppervlakte, de grootste steden en hoe populair een land echt is bij Nederlandse reizigers.\n\nDaarom bouwde ik mijn eigen desktopapp, met een interactieve wereldkaart waarop je kunt inzoomen, live data van de World Bank en GeoNames en een eigen foto per land. De interface is beschikbaar in zes talen.\n\nAlles is in Rust gebouwd, met een Tauri-backend en een Leptos-frontend.",
            ),
            stack: "Rust, Tauri, Leptos",
            link: "https://github.com/lucasr09/wereldreiziger",
        },
    ];

    let interests = vec![
        InterestEntry {
            title: ("Sports", "Sport"),
            text: (
                "I follow a lot of sports, especially darts, football and Formula 1. So working at a darts company feels like a pretty natural fit.",
                "Ik volg veel sport, vooral darten, voetbal en Formule 1. Werken bij een dartbedrijf past daar natuurlijk perfect bij.",
            ),
        },
        InterestEntry {
            title: ("Gaming", "Gamen"),
            text: (
                "I love sandbox games, but I also play a lot of shooters and open world games. I enjoy Europa Universalis so much that I wrote four mission trees for a community mod for EU5.",
                "Ik speel graag sandboxgames, maar ook veel shooters en openwereldgames. Europa Universalis vind ik zo leuk dat ik zelf vier missiebomen heb geschreven voor een community-mod van EU5.",
            ),
        },
        InterestEntry {
            title: ("Geography & geopolitics", "Aardrijkskunde & geopolitiek"),
            text: (
                "I'm fascinated by countries, borders and how they relate to each other. I like reading about what is happening in the world and why.",
                "Landen, grenzen en de verhoudingen tussen landen fascineren me. Ik lees graag over wat er in de wereld gebeurt en waarom.",
            ),
        },
        InterestEntry {
            title: ("Travel", "Reizen"),
            text: (
                "I love traveling and discovering new places. That is also why I built Wereldreiziger: to keep track of where I have been and where I still want to go.",
                "Ik ben gek op reizen en het ontdekken van nieuwe plekken. Daarom heb ik ook Wereldreiziger gebouwd, om bij te houden waar ik ben geweest en waar ik nog naartoe wil.",
            ),
        },
    ];

    let experience = vec![
        ExperienceEntry {
            role: ("Junior Developer (full-time)", "Junior Developer (fulltime)"),
            company: "DartCounter",
            dates: ("Nov 2026 – Present", "nov 2026 – heden"),
            bullets: vec![
                (
                    "Building new features for the DartCounter app",
                    "Nieuwe features ontwikkelen voor de DartCounter-app",
                ),
                (
                    "Tracking down and fixing bugs to keep the app stable for its players",
                    "Bugs opsporen en oplossen, zodat de app stabiel blijft voor spelers",
                ),
                (
                    "Working across the stack with Laravel on the backend and Angular on the frontend",
                    "Werken aan zowel de backend (Laravel) als de frontend (Angular)",
                ),
            ],
        },
        ExperienceEntry {
            role: ("Software Developer (internship)", "Software Developer (stage)"),
            company: "ProRail",
            dates: ("Feb 2026 – Jun 2026", "feb 2026 – jun 2026"),
            bullets: vec![
                (
                    "Contributed to the development of the BTD planner",
                    "Meegewerkt aan de ontwikkeling van de BTD-planner",
                ),
                (
                    "Improved usability and system performance",
                    "De gebruiksvriendelijkheid en prestaties van het systeem verbeterd",
                ),
                (
                    "Worked in a Scrum team, taking part in daily standups, retrospectives, week-start planning, and BCB meetings",
                    "Gewerkt in een Scrum-team en deelgenomen aan daily standups, retrospectives, weekstarts en BCB-overleggen",
                ),
            ],
        },
        ExperienceEntry {
            role: ("Cafetaria Staff (part-time)", "Cafetariamedewerker (parttime)"),
            company: "Cafetaria Lunchroom Kolkie",
            dates: ("Nov 2025 – Mar 2026", "nov 2025 – mrt 2026"),
            bullets: vec![(
                "Worked the counter and kitchen at the business behind my Kolkie Website and Kolkie Inkloksysteem projects",
                "Achter de balie en in de keuken gewerkt bij de zaak waarvoor ik ook de Kolkie Website en het Kolkie Inkloksysteem heb gebouwd",
            )],
        },
        ExperienceEntry {
            role: ("Software Developer (internship)", "Software Developer (stage)"),
            company: "Coldenhove Papierfabriek",
            dates: ("Feb 2025 – Jul 2025", "feb 2025 – jul 2025"),
            bullets: vec![
                (
                    "Built a new company-wide phone directory application",
                    "Een nieuwe telefoonlijst-applicatie voor het hele bedrijf gebouwd",
                ),
                (
                    "Developed a central launcher tool built around one generic \"open file\" function, so staff could jump to frequently used programs with a click instead of navigating to them manually",
                    "Een centrale launcher gebouwd waarmee collega's veelgebruikte programma's met één klik openen, in plaats van er steeds handmatig naartoe te navigeren",
                ),
                (
                    "Assisted with automation projects",
                    "Meegewerkt aan automatiseringsprojecten",
                ),
            ],
        },
        ExperienceEntry {
            role: ("Software Developer (internship)", "Software Developer (stage)"),
            company: "ACTwebservice",
            dates: ("Sep 2023 – Feb 2024", "sep 2023 – feb 2024"),
            bullets: vec![
                (
                    "Built websites for external clients using Duda",
                    "Websites gebouwd in Duda voor externe klanten",
                ),
                ("Handled client conversations", "Klantgesprekken gevoerd"),
                (
                    "Designed and set up databases",
                    "Databases ontworpen en opgezet",
                ),
            ],
        },
        ExperienceEntry {
            role: ("Snack Bar Staff (seasonal)", "Snackbarmedewerker (seizoenswerk)"),
            company: "Ardoer Camping De Jutberg",
            dates: ("2023 – 2026", "2023 – 2026"),
            bullets: vec![
                (
                    "Worked at the campsite's snack bar every season from March to October, alongside school",
                    "Naast school elk seizoen van maart tot oktober in de snackbar van de camping gewerkt",
                ),
                (
                    "Also stepped in as an assistant cook from time to time",
                    "Af en toe ook ingezet als hulpkok",
                ),
                (
                    "Ran the bread service, which inspired my bon-generator prototype",
                    "De broodjesservice gedraaid, waar ook het idee voor mijn bon-generator vandaan kwam",
                ),
                (
                    "Lifeguard at the campsite in the summer of 2021",
                    "Badmeester op de camping in de zomer van 2021",
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
        "Laravel",
        "Angular",
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
    let interests_ref = NodeRef::<html::Section>::new();
    let experience_ref = NodeRef::<html::Section>::new();
    let projects_ref = NodeRef::<html::Section>::new();
    let prototype_ref = NodeRef::<html::Section>::new();
    let skills_ref = NodeRef::<html::Section>::new();
    let contact_ref = NodeRef::<html::Section>::new();

    let (about_revealed, about_in_view) = use_section_observer(about_ref);
    let (interests_revealed, _) = use_section_observer(interests_ref);
    let (experience_revealed, experience_in_view) = use_section_observer(experience_ref);
    let (projects_revealed, projects_in_view) = use_section_observer(projects_ref);
    let (prototype_revealed, prototype_in_view) = use_section_observer(prototype_ref);
    let (skills_revealed, skills_in_view) = use_section_observer(skills_ref);
    let (contact_revealed, contact_in_view) = use_section_observer(contact_ref);

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

    // Easter egg: 3x snel op het logo klikken = "180!". Elke klik krijgt een
    // volgnummer; de reset-timer telt alleen als er sindsdien niet opnieuw
    // geklikt is, zodat alleen snelle opeenvolgende klikken meetellen.
    let (show_180, set_show_180) = signal(false);
    let logo_clicks = StoredValue::new(0u8);
    let click_seq = StoredValue::new(0u32);
    let popup_seq = StoredValue::new(0u32);
    let on_logo_click = move |_| {
        click_seq.update_value(|n| *n = n.wrapping_add(1));
        let seq = click_seq.get_value();
        logo_clicks.update_value(|c| *c += 1);

        if logo_clicks.get_value() >= 3 {
            logo_clicks.set_value(0);
            set_show_180.set(true);
            popup_seq.update_value(|n| *n = n.wrapping_add(1));
            let shown = popup_seq.get_value();
            set_timeout(
                move || {
                    if popup_seq.get_value() == shown {
                        set_show_180.set(false);
                    }
                },
                Duration::from_millis(1800),
            );
        } else {
            set_timeout(
                move || {
                    if click_seq.get_value() == seq {
                        logo_clicks.set_value(0);
                    }
                },
                Duration::from_millis(700),
            );
        }
    };

    view! {
        <div class="site-shell">
            <header class="topbar" class:scrolled=nav_scrolled>
                <div class="container nav-inner">
                    <div class="logo-wrap">
                        <a class="logo" href="#home" on:click=on_logo_click>"LR"</a>
                        <span class="logo-180" class:show=move || show_180.get() aria-hidden="true">"180!"</span>
                    </div>

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
                            {move || t(lang.get(), "Junior Developer at DartCounter, Gelderland", "Junior Developer bij DartCounter, Gelderland")}
                        </p>
                        <div class="hero-board">
                            <Dartboard />
                        </div>
                    </div>
                </section>

                <section id="about" class="section" class:revealed=move || about_revealed.get() node_ref=about_ref>
                    <div class="container split">
                        <div>
                            <p class="section-label">{move || t(lang.get(), "About", "Over mij")}</p>
                            <h2>{move || t(
                                lang.get(),
                                "Developer focused on building solid software without unnecessary complexity.",
                                "Developer die degelijke software bouwt, zonder onnodige complexiteit.",
                            )}</h2>
                        </div>
                        <div class="content-card">
                            <p>
                                {move || t(
                                    lang.get(),
                                    "I enjoy building software that is practical, structured, and technically honest.",
                                    "Ik bouw graag software die praktisch en goed gestructureerd is, en die gewoon doet wat hij moet doen.",
                                )}
                            </p>
                            <p>
                                {move || t(
                                    lang.get(),
                                    "My main interests are Rust, backend development, web applications, and interactive projects such as games.",
                                    "Mijn interesse ligt vooral bij Rust, backend-ontwikkeling, webapplicaties en interactieve projecten zoals games.",
                                )}
                            </p>
                            <p>
                                {move || t(
                                    lang.get(),
                                    "I care about clean code, clear architecture, and building things that are useful rather than overdesigned.",
                                    "Ik vind schone code en een duidelijke opbouw belangrijk, en bouw liever iets wat echt nuttig is dan iets wat onnodig ingewikkeld is.",
                                )}
                            </p>
                            <p class="about-education">
                                {move || t(
                                    lang.get(),
                                    "In 2026 I graduated in Software Development (MBO level 4) from Astrum College in Velp, and from November I will be working full-time as a Junior Developer at DartCounter.",
                                    "In 2026 heb ik de mbo 4-opleiding Software Development afgerond aan het Astrum College in Velp. Vanaf november werk ik fulltime als Junior Developer bij DartCounter.",
                                )}
                            </p>
                        </div>
                    </div>
                </section>

                <section id="interests" class="section" class:revealed=move || interests_revealed.get() node_ref=interests_ref>
                    <div class="container">
                        <p class="section-label">{move || t(lang.get(), "Beyond code", "Buiten de code")}</p>
                        <h2>{move || t(
                            lang.get(),
                            "What I enjoy when I'm not programming",
                            "Wat ik leuk vind naast het programmeren",
                        )}</h2>
                        <div class="interests-grid">
                            {move || interests
                                .iter()
                                .map(|i| view! {
                                    <article class="interest-card">
                                        <h3>{t(lang.get(), i.title.0, i.title.1)}</h3>
                                        <p>{t(lang.get(), i.text.0, i.text.1)}</p>
                                    </article>
                                })
                                .collect_view()}
                        </div>
                    </div>
                </section>

                <section id="experience" class="section" class:revealed=move || experience_revealed.get() node_ref=experience_ref>
                    <div class="container">
                        <p class="section-label">{move || t(lang.get(), "Experience", "Ervaring")}</p>
                        <h2>{move || t(
                            lang.get(),
                            "From four internships and work alongside my studies to my first full-time developer role",
                            "Van vier stages en bijbanen tijdens mijn studie naar mijn eerste vaste baan als developer",
                        )}</h2>
                        <div class="timeline">
                            {move || experience
                                .iter()
                                .map(|entry| {
                                    let role = t(lang.get(), entry.role.0, entry.role.1);
                                    let dates = t(lang.get(), entry.dates.0, entry.dates.1);
                                    let bullets = entry.bullets.clone();
                                    // Stages in dartbord-rood, echte banen in groen.
                                    let internship = entry.role.0.contains("(internship)");
                                    view! {
                                        <div class="timeline-item" class:internship=internship>
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
                                            <div class="project-desc">
                                                {description.split("\n\n").map(|para| view! { <p>{para}</p> }).collect_view()}
                                            </div>
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
                            "Prototypes en proofs of concept",
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
                                            <div class="project-desc">
                                                {description.split("\n\n").map(|para| view! { <p>{para}</p> }).collect_view()}
                                            </div>
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
                            <h2>{move || t(lang.get(), "Let's build something solid.", "Zin om samen iets moois te bouwen?")}</h2>
                            <p class="contact-copy">
                                {move || t(
                                    lang.get(),
                                    "For projects, collaboration, or simply a good technical conversation.",
                                    "Voor projecten, samenwerking of gewoon een goed gesprek over techniek.",
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

            <footer class="site-footer">
                <div class="container footer-inner">
                    <p class="footer-name">
                        <span class="footer-bull" aria-hidden="true"></span>
                        "© Lucas Rensen"
                    </p>
                    <p class="footer-built">{move || t(lang.get(), "Built with Rust and Leptos", "Gebouwd met Rust en Leptos")}</p>
                </div>
            </footer>
        </div>
    }
}
