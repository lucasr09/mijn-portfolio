# Mijn Portfolio

Een snelle en minimalistische portfolio website gebouwd met Rust en Leptos.

## Over dit project

Dit project is mijn persoonlijke portfolio waarin ik mijn projecten, skills en contactinformatie toon.
De focus ligt op performance, eenvoud en een duidelijke presentatie van mijn werk.

## Stack

* Rust
* Leptos
* Trunk (build tool)
* WebAssembly (WASM)

## Features

* Snelle laadtijden via WebAssembly
* Component-based UI met Leptos
* Responsive design
* Simpele en duidelijke structuur

## Lokaal draaien

Zorg dat Rust en Trunk geïnstalleerd zijn.

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

Start de development server:

```bash
trunk serve
```

Open daarna:

http://127.0.0.1:3000

## Projectstructuur

```
src/
  lib.rs    # de hele App: layout, secties, taal- en thema-logica
  main.rs   # mount de App in de body
```

## Live zetten via Vercel

Dit project bouwt met Trunk naar een map met pure statische bestanden
(`dist/`), dus Vercel hoeft zelf geen Rust te draaien — het serveert alleen
het eindresultaat. De `vercel.json` in deze map regelt dat Vercel tijdens
elke build eerst Rust + Trunk installeert en dan `trunk build --release`
draait.

1. Ga naar [vercel.com](https://vercel.com) en importeer deze GitHub-repo.
2. Omdat het project niet in de root van de repo staat, zet je bij
   **Root Directory** in de projectinstellingen: `mijn-portfolio`.
3. Build command en output directory staan al goed via `vercel.json` — hoef
   je verder niks aan te veranderen.
4. Deploy. De eerste build duurt iets langer (~2-3 minuten extra) omdat
   Rust en Trunk geïnstalleerd moeten worden.

Daarna is updaten net zo simpel als bij elk ander Vercel-project: gewoon
`git push` naar `main`, en Vercel bouwt en deployt automatisch.

## Doel

Het doel van dit project is:

* mijn programmeervaardigheden tonen
* ervaring opdoen met Rust en frontend development
* een overzicht geven van mijn projecten

## Toekomstige uitbreidingen

* Extra projecten toevoegen
* Verbeterde styling
* Mogelijk blog of CMS-functionaliteit

## Contact

* GitHub: https://github.com/lucasr09
* Email: [lucasrensen@outlook.com](mailto:lucasrensen@outlook.com)
