# Iterácia 08 - HTMX Todo App

> **⚠️ Pozor: Verzia tohto zadania nemusí byť aktuálna!**
>
> **Aktuálnu verziu nájdete v [issues](https://gitlab.fi.muni.cz/pv281/pv281-iterations-2023/-/issues/).**

> Autor zadania a vzorovej implementácie: [Hugo Adamove](https://gitlab.fi.muni.cz/xadamove)

V tejto iterácii vytvoríte cliché Todo aplikáciu, ktorá vám môže pomôcť osvojiť si tvorbu jednoduchých reaktívnych aplikácií bez pomoci JavaScriptu.

Kostra obsahuje hotovú vrstvu interakcie s databázou a jednoduché demo základného použitia htmx. Vašou úlohou bude pridať vlastné Actix endpointy, šablóny a logiku aplikácie potrebnú pre splnenie požadovanej funkcionality.

## Setup

1. Vytvorím súbor `.env` s vhodným obsahom (skopírujte `.env.example`).
2. Pustím `cargo run` - automaticky sa vytvorí databáza a vykonajú sa migrácie.

> Práca s databázou nie je primárnym cieľom iterácie, preto pre jednoduchosť kostra používa SQLite, na ktorej setup nepotrebujeme docker.

## Úlohy a hodnotenie

- Zoznam všetkých položiek, pridávanie nových položiek - **2 body**
- Schopnosť označiť položku ako splnenú - **1 bod**
- Schopnosť upraviť (text) a odstrániť položku - **1 bod**
- Vymazať všetky splnené položky - **1 bod**

### Požiadavky

- Pre uznanie bodov za úlohy **musia byť všetky zmeny perzistentné** (uložené v databáze tak, aby boli viditeľné po refreshy stránky).
- Pre uznanie bodov odovzdané riešenie **nemôže obsahovať JS kód** (okrem malých fragmentov napr. pri použití `hx-on`).
- Vzhľad aplikácie je na vás, mala by sa však dať jednoducho používať (napr. označenie itemov ako dokončených nemusí byť checkbox).

## Tipy

Ak chcete vidieť svoje zmeny hneď počas práce, [cargo-watch](https://crates.io/crates/cargo-watch) crate prekompiluje aplikáciu pri každej zmene:

```bash
cargo install cargo-watch
cargo watch -x run --ignore 'db/*'
```
> **Poznámka**: `--ignore 'db/*'` je potrebné, aby sa zabránilo prekompilovaniu pri každej zmene databázy.

Ak používate VSCode, môžete použiť rozšírenie [SQLite Viewer](https://marketplace.visualstudio.com/items?itemName=qwtel.sqlite-viewer) pre inšpekciu databázy priamo z editora.

### Zdroje

- Užitočná môže byť dokumentácia [Actix Web](https://actix.rs/docs/), obzvlášť sekcie [Extractors](https://actix.rs/docs/extractors/) a [Handlers](https://actix.rs/docs/handlers/).
- [htmx dokumentácia](https://htmx.org/docs) - je pomerne dlhá, prečítanie [introduction](https://htmx.org/docs/#introduction), [triggers](https://htmx.org/docs/#triggers), [targets](https://htmx.org/docs/#targets) a [swapping](https://htmx.org/docs/#swapping) by vám malo stačiť.
- [Askama](https://djc.github.io/askama/)

### Q&A

**Q:** Mám problém XXX s databázou, čo mám robiť?

**A:** Skús manuálne pustiť `sqlx database create` a `sqlx migrate run --source db/migrations`. Ak to nepomôže, napíš na discord.

---

**Q:** Databáza mi na začiatku fungovala, potom som niečo spravil a už nefunguje. Čo teraz?

**A:** Skús `sqlx database reset --source db/migrations`.

---

**Q:** Prečo pipeline nepúšťa žiadne testy?

**A:** Je možné vyrobiť E2E web testy, ktoré overia, že vaša aplikácia spĺňa požadovanú funkcionalitu. Na druhú stranu by vás tieto testy mohli dosť obmedzovať v rámci možností implementácie. Z toho dôvodu sme sa rozhodli do pipeline pridať len build a lint, a funkcionalitu hodnotiť individuálne.

---

**Q:** Nenávidím `.html` súbory a Askamu, existuje nejaká alternatíva?

**A:** Áno! Vrelo doporučujem mkrnúť sa na [Maud](https://maud.lambda.xyz/). Je šablónovací engine založený na makrách, ktorý vám dovolí vytvárať šablóny v rámci `.rs` súborov. Nebojte sa ho v iterácií použiť.

## Changelog