---
name: vvb-jcode-fork
description: Använd vid ALLT arbete i detta repo (vvb-1-forken av jcode) — uppdatering från upstream, rebase av crab/sv-dropdown, konfliktlösning i de svenska översättningarna, byggverifiering och push. Använd särskilt när användaren säger "uppdatera jcode".
---

# VVB:s fork av jcode

Detta är **inte** upstream-repot. Det är Kristians fork med egna patchar ovanpå.

## Remotes — kritiskt

| Remote   | URL                                  | Får pushas till |
| -------- | ------------------------------------ | --------------- |
| `origin` | `https://github.com/1jehuang/jcode`  | **ALDRIG**      |
| `fork`   | `https://github.com/vvb-1/jcode`     | Ja              |

`origin` är underhållarens repo. En `pre_tool`-hook (`~/bin/jcode-tool-policy`)
blockerar push dit, men förlita dig inte på den. Kontrollera själv.

Arbetsbranch: **`crab/sv-dropdown`**. Lämna aldrig användaren kvar på `master`.

## Egna patchar som måste bevaras

1. `i18n: översätt slash-kommandobeskrivningar till svenska (VVB)`
   — `crates/jcode-tui/src/tui/app/state_ui_input_helpers.rs`
2. `tui: fuzzy-matcha slash-kommandon även mot beskrivningen`
3. `docs: define self-dev agent workflow`
4. `fix: use repo working dir for self-dev` — `src/cli/selfdev.rs`

## Flödet för "uppdatera jcode"

```bash
git checkout master
git pull --ff-only
git checkout crab/sv-dropdown
git rebase master
# lös konflikter, se nedan
selfdev build target=tui        # måste ge exit 0
git push --force-with-lease fork crab/sv-dropdown
```

Kör **aldrig** `git pull` medan HEAD står på `crab/sv-dropdown`.
Kör **aldrig** `git rebase --abort` utan att fråga.

## Återkommande konflikter och hur de löses

### `state_ui_input_helpers.rs`
Upstream lägger till nya slash-kommandon på engelska. Behåll **vår** sida
(svenska) och **översätt de nya kommandona** i stället för att tappa dem.
Exempel: upstream lade till `/update-sim`, vilket blev
`"Förhandsvisa uppdaterings-UI säkert (Alt+_)"`.

Checklista: jämför antalet `RegisteredCommand::`-rader före och efter. Blir
det färre har ett upstream-kommando fallit bort.

**Hjälptexter finns på TVÅ ställen.** Den statiska tabellen
`REGISTERED_COMMANDS` är bara hälften. Underkommandonas texter byggs på plats
i `get_suggestions_for()`:

```rust
("/memory on".into(), "Slå på minne för den här sessionen"),
```

Den ursprungliga i18n-patchen tog bara tabellen, så 102 texter stod kvar på
engelska mitt i en svensk meny. Rustfmt bryter dessutom långa poster över två
rader, vilket kräver ett andra sökmönster:

```rust
(
    "/selfdev enter ".into(),
    "Öppna en self-dev-session med en prompt",
),
```

Kontrollera efter varje rebase att inga engelska hjälptexter smugit in:

```bash
grep -nE '"/[^"]+"[[:space:]]*\.into\(\)' \
  crates/jcode-tui/src/tui/app/state_ui_input_helpers.rs
```

### `src/cli/selfdev.rs`
Upstream ändrar signaturen på `run_tui_client`. Vår patch skickar
`selfdev_remote_working_dir(&repo_dir)` som `remote_working_dir` i stället för
`None`. Behåll det argumentet och lägg till upstreams nya argument
(t.ex. `update_sim: bool`) på rätt position. Verifiera mot signaturen i
`src/cli/tui_launch.rs`.

## Verifiering

```bash
selfdev build target=tui
# fallback om coordinated build saknas:
scripts/dev_cargo.sh build --profile selfdev -p jcode --bin jcode
```

En grön kompilering räcker inte för UI-ändringar. Kontrollera att
slash-menyn faktiskt visar svenska efter `selfdev reload`.

## Hooks på denna maskin

Aktiva i `~/.jcode/config.toml`:

| Hook | Skript | Typ |
| --- | --- | --- |
| `pre_tool` | `~/bin/jcode-tool-policy` | gate, kan blockera (exit 2) |
| `session_start` | `~/bin/jcode-session-start` | observer, blockerar aldrig |

`pre_tool` blockerar push till remotes som inte tillhör `vvb-1`, rent
`git push --force`, historikförstörande git-kommandon och klassiskt
destruktiva shell-kommandon.

### Verifiera alltid efter ändring

```bash
bash ~/bin/jcode-hooks-verify
```

Den kontrollerar config, att skripten finns, att exec-biten är satt, och kör
båda beteendesviterna (31 + 7 fall).

### På VVB-DEV (Windows) körs hookarna i WSL

Här är jcode en **Windows**-binär medan hookarna är bash-skript i **WSL**.
Det gav fyra fel som alla såg ut som fungerande konfiguration. Grinden
rapporterade aldrig något, den släppte bara igenom allt.

Konfigurationen måste gå via wrappern, inte direkt till `wsl.exe`:

```toml
pre_tool = ["C:/Users/Dev/.jcode/verktyg/kor-hook.cmd jcode-tool-policy"]
session_start = ["C:/Users/Dev/.jcode/verktyg/kor-hook.cmd jcode-session-start"]
```

1. **WSL vidarebefordrar inte Windows-miljövariabler.** Utan `WSLENV` såg
   skriptet ett tomt `JCODE_HOOK_TOOL_NAME`, föll ur på sin `case`-rad och
   svarade exit 0. Wrappern `kor-hook.cmd` sätter `WSLENV`.
2. **`JCODE_HOOK_CWD` kommer som `C:\...`.** `[ -d ]` hittar den aldrig, så
   policyn föll ur på sin egen skyddsrad. Skripten kör nu `wslpath -u`.
3. **jq fanns inte i PATH.** Hooken startas inte av ett inloggningsskal, så
   `~/.profile` körs aldrig och `~/.local/bin` saknas. Skriptet lägger nu
   till sökvägen själv, och fallbacken utan jq extraherar `.command` med sed
   i stället för att låta hela JSON-strängen passera som kommando.
4. **CRLF gör skripten okörbara.** Lagrades de med CRLF gav bash
   `syntax error near unexpected token $'in\r'`. `.gitattributes` tvingar nu
   LF för `.jcode/bin/*`. Kontrollera efter varje redigering från Windows,
   flera editorer återinför CRLF.

Verifiera alltid med ett **skarpt** anrop, inte bara testsviten:

```bash
git reset --hard HEAD    # ska ge "Tool call blocked by pre_tool hook"
git status               # ska fungera normalt
```

### Fallgropar som faktiskt inträffade 2026-08-27

1. **Saknad exec-bit failar open.** jcode loggar en varning och fortsätter, så
   en trasig hook ser ut att fungera. Därför kontrollerar `jcode-hooks-verify`
   exec-biten explicit.
2. **Skriv aldrig `${VAR:-{}}` i bash.** Klammern tolkas fel och ger en extra
   `}`, vilket gjorde händelseloggen till ogiltig JSON.
3. **Heredoc-kroppar är data, inte kommandon.** Policyn blockerade först
   dokumentation som bara *nämnde* farliga kommandon. Den klipper nu bort
   heredoc-kroppar före matchning, men kontrollerar fortfarande kommandon
   efter avslutaren.

## Versionshanterade skript

Hook- och policyskripten kors fran `~/bin`, men versionshanteras i
`.jcode/bin/`. De styr en gate som kan blockera verktygsanrop och en policy
som skyddar sju GitHub-repon, sa en trasig andring maste ga att rulla
tillbaka.

```bash
vvb-bin-sync            # kontrollera att repo och ~/bin ar identiska
vvb-bin-sync --to-repo  # efter en andring i ~/bin
vvb-bin-sync --to-home  # efter en git pull
```

Kor `vvb-bin-sync` efter varje andring. En versionshanterad kopia som
driftat isar fran verkligheten ar varre an ingen kopia alls: historiken
beskriver da kod som ingen kor.

### Notisfallgrop

`jcode-session-start` matchar **exakt sokvag**, inte glob. Ett tidigare
monster `*/jcode-audit*` traffade testsvitens temporara `jcode-audit-fake`
och skickade 20 falska notiser till anvandaren.

Kod som notifierar ska alltid ha ett testlage. Hooken respekterar
`JCODE_HOOK_TEST=1`, som testsviten satter.
