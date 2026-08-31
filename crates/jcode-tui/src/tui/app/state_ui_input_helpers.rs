use super::*;
use crate::tui::core;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

#[derive(Clone, Copy)]
struct RegisteredCommand {
    name: &'static str,
    help: &'static str,
    hidden: bool,
}

impl RegisteredCommand {
    const fn public(name: &'static str, help: &'static str) -> Self {
        Self {
            name,
            help,
            hidden: false,
        }
    }

    const fn remote(name: &'static str, help: &'static str) -> Self {
        Self {
            name,
            help,
            hidden: false,
        }
    }

    const fn hidden(name: &'static str, help: &'static str) -> Self {
        Self {
            name,
            help,
            hidden: true,
        }
    }
}

const REGISTERED_COMMANDS: &[RegisteredCommand] = &[
    RegisteredCommand::public("/help", "Visa hjälp och kortkommandon"),
    RegisteredCommand::public("/?", "Visa hjälp och kortkommandon"),
    RegisteredCommand::public("/commands", "Alias för /help"),
    RegisteredCommand::public("/model", "Lista eller byt modell"),
    RegisteredCommand::public("/models", "Alias för /model"),
    RegisteredCommand::public(
        "/provider-test-coverage",
        "Visa live-testbevis för aktuell provider/modell",
    ),
    RegisteredCommand::hidden("/model-status", "Alias för /provider-test-coverage"),
    RegisteredCommand::public("/refresh-model-list", "Uppdatera provider-modellkataloger"),
    RegisteredCommand::public("/agents", "Konfigurera modeller för agentroller"),
    RegisteredCommand::public(
        "/swarm-prompt",
        "Öppna aktiv swarm-routingprompt i din editor",
    ),
    RegisteredCommand::public("/subagent", "Starta en subagent manuellt"),
    RegisteredCommand::public("/observe", "Visa senaste verktygskontext i sidopanelen"),
    RegisteredCommand::public("/todos", "Visa sessionens todo-lista som kort i chatten"),
    RegisteredCommand::hidden("/todo", "Alias för /todos"),
    RegisteredCommand::public("/splitview", "Spegla aktuell chatt i sidopanelen"),
    RegisteredCommand::public("/split-view", "Alias för /splitview"),
    RegisteredCommand::public("/btw", "Ställ en sidofråga i sidopanelen"),
    RegisteredCommand::public("/ssh", "Anslut till fjärrmaskin via system-SSH"),
    RegisteredCommand::public("/git", "Visa git-status för sessionens arbetskatalog"),
    RegisteredCommand::public("/colors", "Lista, konfigurera och betygsätt alla TUI-färger"),
    RegisteredCommand::hidden("/color", "Alias för /colors"),
    RegisteredCommand::public("/hotkeys", "Lista kortkommandon med din personliga användning"),
    RegisteredCommand::public("/terminal-setup", "Fixa Shift+Enter-radbrytningar"),
    RegisteredCommand::public("/commit", "Skapa logiska commits från aktuella ändringar"),
    RegisteredCommand::public(
        "/commit-push",
        "Skapa logiska commits från aktuella ändringar, pusha sedan",
    ),
    RegisteredCommand::hidden("/commit-and-push", "Alias för /commit-push"),
    RegisteredCommand::public(
        "/fast-release",
        "Publicera Linux direkt från varm selfdev-cache; CI lägger till övriga plattformar",
    ),
    RegisteredCommand::public(
        "/fast-macos-release",
        "Publicera förberett macOS arm64-bygge direkt; CI lägger till övriga plattformar",
    ),
    RegisteredCommand::public("/remote", "Nå denna session från en annan maskin"),
    RegisteredCommand::public(
        "/remote-release",
        "Pusha release-taggen direkt; CI bygger och publicerar alla plattformar",
    ),
    RegisteredCommand::hidden("/cut-release", "Alias för /fast-release"),
    RegisteredCommand::hidden("/commit-push-release", "Alias för /cut-release"),
    RegisteredCommand::public(
        "/triage",
        "Triagea nya GitHub-issues och fixa de säkra automatiskt",
    ),
    RegisteredCommand::public("/transcript", "Öppna aktuell sessionstranskriptfil"),
    RegisteredCommand::public("/subagent-model", "Visa/ändra subagent-modellpolicy"),
    RegisteredCommand::public("/autoreview", "Visa/aktivera automatisk granskning vid turn-slut"),
    RegisteredCommand::public("/autojudge", "Visa/aktivera automatisk bedömning vid turn-slut"),
    RegisteredCommand::public("/review", "Starta en engångsgranskning direkt"),
    RegisteredCommand::public("/judge", "Starta en engångsbedömning direkt"),
    RegisteredCommand::public("/effort", crate::tui::keybind::EFFORT_HELP),
    RegisteredCommand::public("/fast", "Aktivera/avaktivera snabbläge"),
    RegisteredCommand::public("/transport", "Visa/ändra anslutningstransport"),
    RegisteredCommand::public("/alignment", "Visa/ändra standardtextjustering"),
    RegisteredCommand::public(
        "/compact-notifications",
        "Visa/aktivera kompakta swarm-/filaktivitetsnotiser",
    ),
    RegisteredCommand::public(
        "/show-agentgrep-output",
        "Visa/aktivera full agentgrep-sökutdata i chatten",
    ),
    RegisteredCommand::public(
        "/tool-call-details",
        "Visa/aktivera nedtonade tekniska detaljer på verktygsrader",
    ),
    RegisteredCommand::public(
        "/thinking-display",
        "Visa/dölj modellens tänkande (av/full/aktuell)",
    ),
    RegisteredCommand::hidden("/thinking", "Alias för /thinking-display"),
    RegisteredCommand::hidden("/reasoning", "Alias för /thinking-display"),
    RegisteredCommand::public("/cancel", "Avbryt aktuell prompt eller operation"),
    RegisteredCommand::public("/clear", "Rensa konversationshistorik"),
    RegisteredCommand::public("/cls", "Rensa vyn endast, behåll kontext"),
    RegisteredCommand::hidden("/clear-view", "Alias för /cls"),
    RegisteredCommand::public("/rewind", "Spola tillbaka konversationen till tidigare meddelande"),
    RegisteredCommand::public("/poke", "Puffa modellen att fortsätta med ofärdiga todos"),
    RegisteredCommand::public("/plan", "Skapa plan-endast-svar som plankort"),
    RegisteredCommand::public("/improve", "Förbättra repot automatiskt"),
    RegisteredCommand::public("/refactor", "Kör en säker refaktorloop"),
    RegisteredCommand::public("/compact", "Komprimera kontext"),
    RegisteredCommand::public("/fix", "Återställ när modellen inte kan fortsätta"),
    RegisteredCommand::public("/dictate", "Kör konfigurerat externt diktatkommando"),
    RegisteredCommand::public("/dictation", "Alias för /dictate"),
    RegisteredCommand::public("/memory", "Aktivera/avaktivera minnesfunktion"),
    RegisteredCommand::public("/test", "Verifiera påstående/ändringar med lager-tester"),
    RegisteredCommand::public(
        "/initiatives",
        "Öppna initiativöversikt / återuppta spårade initiativ",
    ),
    RegisteredCommand::public("/goals", "Legacy-alias för /initiatives"),
    RegisteredCommand::public("/swarm", "Aktivera/avaktivera swarm-funktion"),
    RegisteredCommand::public("/overnight", "Kör en övervakad nattkoordinator"),
    RegisteredCommand::public("/context", "Visa fullständig sessionskontextögonblicksbild"),
    RegisteredCommand::public(
        "/skills",
        "Visa laddade skills och jcode-rekommendationer",
    ),
    RegisteredCommand::public("/version", "Visa aktuell version"),
    RegisteredCommand::public("/changelog", "Visa senaste ändringar i detta bygge"),
    RegisteredCommand::public("/info", "Visa sessionsinfo och tokens"),
    RegisteredCommand::public("/usage", "Visa anslutna providers användningsgränser"),
    RegisteredCommand::public(
        "/productivity",
        "Generera delbar användningsrapport + dashboardbild",
    ),
    RegisteredCommand::public("/wrapped", "Alias för /productivity"),
    RegisteredCommand::public("/feedback", "Skicka feedback om jcode"),
    RegisteredCommand::public("/telemetry", "Visa eller ändra vad jcode skickar"),
    RegisteredCommand::public("/support", "Mejla support med förifylld diagnostik"),
    RegisteredCommand::public("/subscription", "Visa jcode-prenumerationsstatus"),
    RegisteredCommand::public("/subscribe", "Varför och hur man prenumererar på jcode"),
    RegisteredCommand::public("/config", "Visa eller redigera konfiguration"),
    RegisteredCommand::public("/log", "Markera aktuell plats i jcode-loggarna"),
    RegisteredCommand::public(
        "/keys",
        "Visa tangentkonflikter med din terminal och OS (/keys refresh för omskanning)",
    ),
    RegisteredCommand::hidden("/keybindings", "Alias för /keys"),
    RegisteredCommand::public(
        "/diff",
        "Växla eller sätt diff-läge (av/inline/full/pinned/fil)",
    ),
    RegisteredCommand::public(
        "/onboarding-preview",
        "Förhandsvisa första-körnings-skärmen",
    ),
    RegisteredCommand::public(
        "/onboarding-sim",
        "Gå igenom alla första-körnings-skärmar (Alt+5 återställ, Cmd+5 växla)",
    ),
    RegisteredCommand::public("/reload", "Ladda om till senaste tillgängliga binär"),
    RegisteredCommand::public("/restart", "Starta om med aktuell binär"),
    RegisteredCommand::public("/rebuild", "Bakgrundsbygg och auto-ladda om"),
    RegisteredCommand::public("/selfdev", "Öppna en ny self-dev jcode-session"),
    RegisteredCommand::public("/update", "Bakgrundsuppdatering och auto-ladda om"),
    RegisteredCommand::public("/update-sim", "Förhandsvisa uppdaterings-UI säkert (Alt+_)"),
    RegisteredCommand::public("/resume", "Öppna sessionsväljare"),
    RegisteredCommand::public("/sessions", "Alias för /resume"),
    RegisteredCommand::public("/session", "Alias för /resume"),
    RegisteredCommand::public("/active", "Hantera live-sessioner (arbetar vs redo)"),
    RegisteredCommand::public("/catchup", "Öppna Catch Up-väljare"),
    RegisteredCommand::public("/back", "Återvänd till föregående Catch Up-session"),
    RegisteredCommand::public("/save", "Bokmärk session för enkel åtkomst"),
    RegisteredCommand::public("/unsave", "Ta bort bokmärke från session"),
    RegisteredCommand::public("/rename", "Byt namn på aktuell session"),
    RegisteredCommand::public("/fork", "Forka session till nytt fönster (valfri prompt)"),
    RegisteredCommand::hidden("/split", "Alias för /fork"),
    RegisteredCommand::public("/transfer", "Komprimera kontext till färsk handoff-session"),
    RegisteredCommand::public("/workspace", "Niri-stil sessionsarbetsyta"),
    RegisteredCommand::public("/quit", "Avsluta jcode"),
    RegisteredCommand::public("/auth", "Visa autentiseringsstatus"),
    RegisteredCommand::public("/login", "Logga in på en provider"),
    RegisteredCommand::public("/logout", "Logga ut från en provider"),
    RegisteredCommand::public("/account", "Öppna kombinerad kontoväljare"),
    RegisteredCommand::public("/accounts", "Alias för /account"),
    RegisteredCommand::public("/cache", "Visa cache-statistik eller sätt cache-TTL"),
    RegisteredCommand::public("/debug-visual", "Växla visuell debug-overlay"),
    RegisteredCommand::public("/screenshot-mode", "Växla skärmbildsläge"),
    RegisteredCommand::public("/screenshot", "Fånga skärmbild av debug-state"),
    RegisteredCommand::public("/record", "Spela in demo-fångst"),
    RegisteredCommand::remote("/client-reload", "Tvinga omladdning av klientbinär"),
    RegisteredCommand::remote("/server-reload", "Tvinga omladdning av serverbinär"),
    RegisteredCommand::remote(
        "/continue",
        "Fortsätt alla avbrutna live-sessioner som skulle auto-återupptas",
    ),
    RegisteredCommand::remote("/resumeall", "Alias för /continue"),
    RegisteredCommand::hidden("/resume-all", "Alias för /continue"),
    RegisteredCommand::hidden("/z", "Hemligt premium-läge-kommando"),
    RegisteredCommand::hidden("/zz", "Hemligt premium-läge-kommando"),
    RegisteredCommand::hidden("/zzz", "Hemligt premium-läge-kommando"),
    RegisteredCommand::hidden("/zstatus", "Hemligt premium-läge-statuskommando"),
];

/// Every non-hidden slash command with its one-line description, in
/// registration order. The `/help` overlay uses this to list commands its
/// hand-written sections have not covered, so a newly registered command can
/// never be invisible to users.
pub(crate) fn registered_command_entries() -> impl Iterator<Item = (&'static str, &'static str)> {
    REGISTERED_COMMANDS
        .iter()
        .filter(|command| !command.hidden)
        .map(|command| (command.name, command.help))
}

impl App {
    /// Find word boundary going backward (for Ctrl+W, Alt+B)
    pub(super) fn find_word_boundary_back(&self) -> usize {
        if self.cursor_pos == 0 {
            return 0;
        }
        let mut pos = self.cursor_pos;

        // Move back one char
        pos = core::prev_char_boundary(&self.input, pos);

        // Skip trailing whitespace
        while pos > 0 {
            let ch = self.input[pos..].chars().next().unwrap_or(' ');
            if !ch.is_whitespace() {
                break;
            }
            pos = core::prev_char_boundary(&self.input, pos);
        }

        // Skip word characters
        while pos > 0 {
            let prev = core::prev_char_boundary(&self.input, pos);
            let ch = self.input[prev..].chars().next().unwrap_or(' ');
            if ch.is_whitespace() {
                break;
            }
            pos = prev;
        }

        pos
    }

    /// Find word boundary going forward (for Alt+F, Alt+D)
    pub(super) fn find_word_boundary_forward(&self) -> usize {
        let len = self.input.len();
        if self.cursor_pos >= len {
            return len;
        }
        let mut pos = self.cursor_pos;

        // Skip current word
        while pos < len {
            let ch = self.input[pos..].chars().next().unwrap_or(' ');
            if ch.is_whitespace() {
                break;
            }
            pos = core::next_char_boundary(&self.input, pos);
        }

        // Skip whitespace
        while pos < len {
            let ch = self.input[pos..].chars().next().unwrap_or(' ');
            if !ch.is_whitespace() {
                break;
            }
            pos = core::next_char_boundary(&self.input, pos);
        }

        pos
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    #[cfg(test)]
    pub(crate) fn set_input_for_test(&mut self, input: impl Into<String>) {
        self.input = input.into();
        self.cursor_pos = self.input.len();
    }

    /// Typo-resistant fuzzy score. Higher is better; `None` means no match.
    /// Delegates to the shared [`crate::tui::fuzzy`] matcher so slash-command
    /// ranking and highlight positions stay in sync.
    pub(super) fn fuzzy_score(needle: &str, haystack: &str) -> Option<i32> {
        crate::tui::fuzzy::fuzzy_score(needle, haystack)
    }

    pub(super) fn rank_suggestions(
        &self,
        needle: &str,
        candidates: Vec<(String, &'static str)>,
    ) -> Vec<(String, &'static str)> {
        let needle = needle.to_lowercase();
        // Bucket 2 = literal prefix matches (exact typing always wins).
        // Bucket 1 = typo-tolerant fuzzy matches on the command name.
        // Bucket 0 = fuzzy matches on the command's description, so typing a
        // word from the help text (e.g. `/bok` -> "Sätter ett bokmärke ...")
        // still surfaces the command.
        let description_needle = needle.trim_start_matches('/').trim();
        let description_query = (description_needle.chars().count() >= 2)
            .then(|| jcode_fuzzy::PreparedTokenQuery::new(description_needle));
        let mut scored: Vec<(u8, i32, String, &'static str)> = Vec::new();
        for (cmd, help) in candidates {
            let lower = cmd.to_lowercase();
            if lower.starts_with(&needle) {
                scored.push((2, i32::MAX, cmd, help));
            } else if let Some(score) = Self::fuzzy_score(&needle, &lower) {
                scored.push((1, score, cmd, help));
            } else if let Some(score) = description_query
                .as_ref()
                .and_then(|query| query.score(&help.to_lowercase()))
            {
                scored.push((0, score, cmd, help));
            }
        }
        scored.sort_by(|a, b| {
            b.0.cmp(&a.0)
                .then_with(|| b.1.cmp(&a.1))
                .then_with(|| a.2.len().cmp(&b.2.len()))
                .then_with(|| a.2.cmp(&b.2))
        });
        scored
            .into_iter()
            .map(|(_, _, cmd, help)| (cmd, help))
            .collect()
    }

    fn command_candidates(&self) -> Vec<(String, &'static str)> {
        if let Some(cache) = self.command_candidates_cache.borrow().as_ref() {
            return cache.candidates.clone();
        }

        fn push_skill_commands(
            commands: &mut Vec<(String, &'static str)>,
            seen: &mut std::collections::HashSet<String>,
            skills: &crate::skill::SkillRegistry,
        ) {
            for skill in skills.list() {
                let command = format!("/{}", skill.name);
                if seen.insert(command.clone()) {
                    commands.push((command, "Activate skill"));
                }
            }
        }

        let mut seen = std::collections::HashSet::new();
        let mut commands: Vec<(String, &'static str)> = REGISTERED_COMMANDS
            .iter()
            .filter(|command| !command.hidden)
            .filter_map(|command| {
                let name = command.name.to_string();
                seen.insert(name.clone()).then_some((name, command.help))
            })
            .collect();

        let skills = self.current_skills_snapshot();
        push_skill_commands(&mut commands, &mut seen, &skills);

        if self.is_remote && !self.remote_skills.is_empty() {
            for skill in &self.remote_skills {
                let command = format!("/{skill}");
                if seen.insert(command.clone()) {
                    commands.push((command, "Activate skill"));
                }
            }
        }

        *self.command_candidates_cache.borrow_mut() = Some(CommandCandidatesCache {
            candidates: commands.clone(),
        });
        commands
    }

    pub(super) fn invalidate_command_candidates_cache(&self) {
        *self.command_candidates_cache.borrow_mut() = None;
    }

    fn model_suggestion_candidates(&self) -> Vec<(String, &'static str)> {
        fn push_unique(
            seen: &mut std::collections::HashSet<String>,
            entries: &mut Vec<String>,
            model: String,
        ) {
            if !model.is_empty() && seen.insert(model.clone()) {
                entries.push(model);
            }
        }

        let mut seen = std::collections::HashSet::new();
        let mut models = Vec::new();

        if self.is_remote {
            if let Some(current) = self.remote_provider_model.clone() {
                push_unique(&mut seen, &mut models, current);
            }

            let routes = if !self.remote_model_options.is_empty() {
                self.remote_model_options.clone()
            } else {
                self.build_remote_model_routes_fallback()
            };

            for route in routes {
                push_unique(&mut seen, &mut models, route.model);
            }

            for model in &self.remote_available_entries {
                push_unique(&mut seen, &mut models, model.clone());
            }
        } else {
            push_unique(&mut seen, &mut models, self.provider.model());
            for model in self.provider.available_models_display() {
                push_unique(&mut seen, &mut models, model);
            }
        }

        models
            .into_iter()
            .map(|model| (format!("/model {}", model), "Switch to model"))
            .collect()
    }

    fn model_provider_suggestion_candidates(&self, model: &str) -> Vec<(String, &'static str)> {
        fn push_unique(
            seen: &mut std::collections::HashSet<String>,
            entries: &mut Vec<(String, &'static str)>,
            command: String,
            help: &'static str,
        ) {
            if !command.is_empty() && seen.insert(command.clone()) {
                entries.push((command, help));
            }
        }

        let model = model.trim();
        if model.is_empty() {
            return Vec::new();
        }
        let Some(openrouter_model) = crate::provider::openrouter_catalog_model_id(model) else {
            return Vec::new();
        };

        let mut seen = std::collections::HashSet::new();
        let mut suggestions = Vec::new();
        push_unique(
            &mut seen,
            &mut suggestions,
            format!("/model {}@auto", openrouter_model),
            "Use automatic OpenRouter provider routing",
        );

        if self.is_remote {
            let routes = if !self.remote_model_options.is_empty() {
                self.remote_model_options.clone()
            } else {
                self.build_remote_model_routes_fallback()
            };

            for route in routes {
                if route.model == model && route.api_method == "openrouter" {
                    let help = if route.provider == "auto" {
                        "Use automatic OpenRouter provider routing"
                    } else {
                        "Pin OpenRouter provider"
                    };
                    push_unique(
                        &mut seen,
                        &mut suggestions,
                        format!("/model {}@{}", openrouter_model, route.provider),
                        help,
                    );
                }
            }
        } else {
            for provider in self.provider.available_providers_for_model(model) {
                push_unique(
                    &mut seen,
                    &mut suggestions,
                    format!("/model {}@{}", openrouter_model, provider),
                    "Pin OpenRouter provider",
                );
            }
        }

        suggestions
    }

    /// Get command suggestions based on current input (or base input for cycling)
    pub(super) fn get_suggestions_for(&self, input: &str) -> Vec<(String, &'static str)> {
        let input = input.trim_start();

        // Only show suggestions when input starts with /
        if !input.starts_with('/') {
            return vec![];
        }

        let prefix = input.to_lowercase();
        let prefix_trimmed = prefix.trim_end();

        if prefix.starts_with("/model ") || prefix.starts_with("/models ") {
            if let Some(model_spec) = input
                .strip_prefix("/model ")
                .or_else(|| input.strip_prefix("/models "))
                && let Some((model, _provider_prefix)) = model_spec.rsplit_once('@')
            {
                let suggestions = self.model_provider_suggestion_candidates(model);
                if !suggestions.is_empty() {
                    return self.rank_suggestions(input, suggestions);
                }
            }

            let suggestions = self.model_suggestion_candidates();
            if suggestions.is_empty() {
                return vec![("/model".into(), "Öppna modellväljaren")];
            }
            return self.rank_suggestions(input, suggestions);
        }

        if prefix.starts_with("/agents ") {
            return self.rank_suggestions(
                input,
                vec![
                    ("/agents swarm".into(), "Konfigurera modell för swarm/underagent"),
                    ("/agents review".into(), "Konfigurera modell för kodgranskning"),
                    ("/agents judge".into(), "Konfigurera modell för bedömare"),
                    ("/agents memory".into(), "Konfigurera modell för minnessidovagn"),
                    ("/agents ambient".into(), "Konfigurera modell för ambient"),
                ],
            );
        }

        if prefix.starts_with("/subagent-model ") {
            return self.rank_suggestions(
                input,
                vec![
                    (
                        "/subagent-model inherit".into(),
                        "Använd den aktiva modellen",
                    ),
                    (
                        "/subagent-model show".into(),
                        "Visa aktuell modellpolicy för underagenter",
                    ),
                ],
            );
        }

        if prefix.starts_with("/autoreview ") {
            return self.rank_suggestions(
                input,
                vec![
                    (
                        "/autoreview status".into(),
                        "Visa aktuell status för automatisk granskning",
                    ),
                    ("/autoreview on".into(), "Slå på automatisk granskning vid turslut"),
                    ("/autoreview off".into(), "Stäng av automatisk granskning vid turslut"),
                    ("/autoreview now".into(), "Starta en granskare direkt"),
                ],
            );
        }

        if prefix_trimmed == "/autoreview" {
            return vec![
                (
                    "/autoreview status".into(),
                    "Visa aktuell status för automatisk granskning",
                ),
                ("/autoreview on".into(), "Slå på automatisk granskning vid turslut"),
                ("/autoreview off".into(), "Stäng av automatisk granskning vid turslut"),
                ("/autoreview now".into(), "Starta en granskare direkt"),
            ];
        }

        if prefix.starts_with("/autojudge ") {
            return self.rank_suggestions(
                input,
                vec![
                    ("/autojudge status".into(), "Visa aktuell status för autobedömning"),
                    ("/autojudge on".into(), "Slå på automatisk bedömning vid turslut"),
                    ("/autojudge off".into(), "Stäng av automatisk bedömning vid turslut"),
                    ("/autojudge now".into(), "Starta en bedömare direkt"),
                ],
            );
        }

        if prefix_trimmed == "/autojudge" {
            return vec![
                ("/autojudge status".into(), "Visa aktuell status för autobedömning"),
                ("/autojudge on".into(), "Slå på automatisk bedömning vid turslut"),
                ("/autojudge off".into(), "Stäng av automatisk bedömning vid turslut"),
                ("/autojudge now".into(), "Starta en bedömare direkt"),
            ];
        }

        if prefix.starts_with("/review ") {
            return self.rank_suggestions(
                input,
                vec![("/review".into(), "Starta en engångsgranskning direkt")],
            );
        }

        if prefix_trimmed == "/review" {
            return vec![("/review".into(), "Starta en engångsgranskning direkt")];
        }

        if prefix.starts_with("/judge ") {
            return self.rank_suggestions(
                input,
                vec![("/judge".into(), "Starta en engångsbedömning direkt")],
            );
        }

        if prefix_trimmed == "/judge" {
            return vec![("/judge".into(), "Starta en engångsbedömning direkt")];
        }

        if prefix_trimmed == "/subagent-model" {
            return vec![
                (
                    "/subagent-model show".into(),
                    "Visa aktuell modellpolicy för underagenter",
                ),
                (
                    "/subagent-model inherit".into(),
                    "Använd den aktiva modellen",
                ),
            ];
        }

        if prefix.starts_with("/subagent ") {
            return self.rank_suggestions(
                input,
                vec![
                    (
                        "/subagent --type general ".into(),
                        "Starta en allmän underagent",
                    ),
                    (
                        "/subagent --model ".into(),
                        "Starta en underagent med angiven modell",
                    ),
                    (
                        "/subagent --continue ".into(),
                        "Återuppta en befintlig underagentsession",
                    ),
                ],
            );
        }

        if prefix_trimmed == "/subagent" {
            return vec![("/subagent ".into(), "Starta en underagent med en prompt")];
        }

        // /model opens the interactive picker, and `/model <name>` supports direct completion.
        if prefix_trimmed == "/model" || prefix_trimmed == "/models" {
            return vec![("/model".into(), "Öppna modellväljaren eller skriv `/model <namn>`")];
        }

        if prefix_trimmed == "/agents" {
            return vec![("/agents".into(), "Öppna väljaren för agentmodeller")];
        }

        if prefix.starts_with("/help ") || prefix.starts_with("/? ") {
            let base = if prefix.starts_with("/? ") {
                "/?"
            } else {
                "/help"
            };
            let topics = self
                .command_candidates()
                .into_iter()
                .map(|(cmd, help)| (format!("{} {}", base, cmd.trim_start_matches('/')), help))
                .collect();
            return self.rank_suggestions(input, topics);
        }

        if prefix.starts_with("/colors ") || prefix.starts_with("/color ") {
            let base = if prefix.starts_with("/color ") {
                "/color"
            } else {
                "/colors"
            };
            let mut suggestions: Vec<(String, &'static str)> = vec![
                (
                    format!("{base} harmony"),
                    "Score the palette and list fixes",
                ),
                (
                    format!("{base} generate #8ab4f8"),
                    "Build a harmonious palette from one seed color",
                ),
                (format!("{base} reset"), "Reset every color to its default"),
                (format!("{base} export"), "Print the palette as config TOML"),
            ];
            suggestions.extend(
                jcode_tui_style::ALL_ROLES
                    .iter()
                    .map(|role| (format!("{base} {} #", role.key()), "Set this color role")),
            );
            return self.rank_suggestions(input, suggestions);
        }

        if prefix.starts_with("/git ") {
            return self.rank_suggestions(
                input,
                vec![("/git status".into(), "Visa branch och arbetskatalogens status")],
            );
        }

        if prefix_trimmed == "/git" {
            return vec![("/git status".into(), "Visa branch och arbetskatalogens status")];
        }

        if prefix.starts_with("/transcript ") {
            return self.rank_suggestions(
                input,
                vec![(
                    "/transcript path".into(),
                    "Skriv ut sökväg till utskrift utan att öppna",
                )],
            );
        }

        if prefix_trimmed == "/transcript" {
            return vec![(
                "/transcript path".into(),
                "Skriv ut sökväg till utskrift utan att öppna",
            )];
        }

        if prefix.starts_with("/effort ") {
            let efforts = ["none", "minimal", "low", "medium", "high", "xhigh", "max"];
            return self.rank_suggestions(
                input,
                efforts
                    .iter()
                    .map(|e| (format!("/effort {}", e), effort_display_label(e)))
                    .collect(),
            );
        }

        if prefix.starts_with("/fast ") {
            let modes = [
                "on",
                "off",
                "status",
                "default on",
                "default off",
                "default status",
            ];
            return self.rank_suggestions(
                input,
                modes.iter().map(|m| (format!("/fast {}", m), *m)).collect(),
            );
        }

        if prefix.starts_with("/transport ") {
            let transports = ["auto", "https", "websocket"];
            return self.rank_suggestions(
                input,
                transports
                    .iter()
                    .map(|t| (format!("/transport {}", t), *t))
                    .collect(),
            );
        }

        if prefix.starts_with("/compact ") {
            let suggestions = vec![
                ("/compact mode".into(), "Visa eller ändra komprimeringsläge"),
                (
                    "/compact mode status".into(),
                    "Visa aktuellt komprimeringsläge",
                ),
                ("/compact mode reactive".into(), "Använd reaktiv komprimering"),
                ("/compact mode proactive".into(), "Använd proaktiv komprimering"),
                ("/compact mode semantic".into(), "Använd semantisk komprimering"),
            ];
            return self.rank_suggestions(input, suggestions);
        }

        if prefix.starts_with("/compact mode ") {
            let modes = ["reactive", "proactive", "semantic"];
            let mut suggestions: Vec<(String, &'static str)> = vec![(
                "/compact mode status".into(),
                "Visa aktuellt komprimeringsläge",
            )];
            suggestions.extend(
                modes
                    .iter()
                    .map(|mode| (format!("/compact mode {}", mode), *mode)),
            );
            return self.rank_suggestions(input, suggestions);
        }

        if prefix.starts_with("/cache ") {
            let suggestions = vec![
                ("/cache stats".into(), "Visa statistik för KV-cachen"),
                ("/cache status".into(), "Alias för /cache stats"),
                ("/cache 1h".into(), "Använd 1 timmes cachelivstid"),
                ("/cache 5m".into(), "Använd 5 minuters cachelivstid"),
            ];
            return self.rank_suggestions(input, suggestions);
        }

        if prefix.starts_with("/login ") || prefix.starts_with("/auth ") {
            let base = if prefix.starts_with("/auth ") {
                "/auth"
            } else {
                "/login"
            };
            let mut suggestions: Vec<(String, &'static str)> = Vec::new();
            if base == "/auth" {
                suggestions.push(("/auth doctor".into(), "Diagnostisera inloggningsproblem hos provider"));
            }
            suggestions.extend(
                crate::provider_catalog::tui_login_providers()
                    .iter()
                    .map(|provider| (format!("{} {}", base, provider.id), provider.menu_detail)),
            );
            return self.rank_suggestions(input, suggestions);
        }

        if prefix.starts_with("/account ") || prefix.starts_with("/accounts ") {
            let mut suggestions = vec![
                ("/account list".into(), "Öppna alla konto- och provideråtgärder"),
                ("/account switch".into(), "Byt aktivt konto via etikett"),
                (
                    "/account default-provider".into(),
                    "Ange föredragen standardprovider",
                ),
                (
                    "/account default-model".into(),
                    "Ange föredragen standardmodell",
                ),
                (
                    "/account openai-compatible settings".into(),
                    "Granska egna OpenAI-kompatibla inställningar",
                ),
                (
                    "/account openai-compatible api-base".into(),
                    "Ange egen OpenAI-kompatibel API-bas",
                ),
            ];
            for provider in crate::provider_catalog::login_providers() {
                suggestions.push((
                    format!("/account {}", provider.id),
                    "Open this provider's account/settings actions",
                ));
                suggestions.push((
                    format!("/account {} settings", provider.id),
                    "Show provider-specific settings",
                ));
                suggestions.push((
                    format!("/account {} login", provider.id),
                    "Start or refresh login for this provider",
                ));
            }
            suggestions.push(("/account claude add".into(), "Lägg till ett nytt Claude-konto"));
            suggestions.push(("/account openai add".into(), "Lägg till ett nytt OpenAI-konto"));
            suggestions.push((
                "/account openai transport".into(),
                "Ange transportval för OpenAI",
            ));
            suggestions.push((
                "/account openai effort".into(),
                "Ange resonemangsnivå för OpenAI",
            ));
            if let Ok(accounts) = crate::auth::claude::list_accounts() {
                for account in accounts {
                    suggestions.push((
                        format!("/account claude switch {}", account.label),
                        "Switch to this Claude account",
                    ));
                }
            }
            if let Ok(accounts) = crate::auth::codex::list_accounts() {
                for account in accounts {
                    suggestions.push((
                        format!("/account openai switch {}", account.label),
                        "Switch to this OpenAI account",
                    ));
                }
            }
            return self.rank_suggestions(input, suggestions);
        }

        if prefix.starts_with("/memory ") {
            return self.rank_suggestions(
                input,
                vec![
                    ("/memory on".into(), "Slå på minne för den här sessionen"),
                    ("/memory off".into(), "Stäng av minne för den här sessionen"),
                    ("/memory status".into(), "Visa status för minnesfunktionen"),
                ],
            );
        }

        if prefix.starts_with("/improve ") {
            return self.rank_suggestions(
                input,
                vec![
                    (
                        "/improve plan".into(),
                        "Skapa en rangordnad förbättringslista utan att ändra kod",
                    ),
                    (
                        "/improve resume".into(),
                        "Återuppta senast sparade förbättringsläge för sessionen",
                    ),
                    (
                        "/improve status".into(),
                        "Visa aktuell förbättringsomgång och härledd status",
                    ),
                    (
                        "/improve stop".into(),
                        "Stoppa förbättringsläget vid nästa säkra punkt",
                    ),
                ],
            );
        }

        if prefix.starts_with("/refactor ") {
            return self.rank_suggestions(
                input,
                vec![
                    (
                        "/refactor plan".into(),
                        "Skapa en rangordnad refaktoreringslista utan att ändra kod",
                    ),
                    (
                        "/refactor resume".into(),
                        "Återuppta senast sparade refaktoreringsläge för sessionen",
                    ),
                    (
                        "/refactor status".into(),
                        "Visa aktuell refaktoreringsomgång och härledd status",
                    ),
                    (
                        "/refactor stop".into(),
                        "Stoppa refaktoreringsläget vid nästa säkra punkt",
                    ),
                ],
            );
        }

        if prefix.starts_with("/swarm ") {
            return self.rank_suggestions(
                input,
                vec![
                    ("/swarm on".into(), "Slå på swarm för den här sessionen"),
                    ("/swarm off".into(), "Stäng av swarm för den här sessionen"),
                    ("/swarm status".into(), "Visa status för swarm-funktionen"),
                ],
            );
        }

        if prefix.starts_with("/overnight ") {
            return self.rank_suggestions(
                input,
                vec![
                    (
                        "/overnight 7".into(),
                        "Starta en 7 timmars övervakad nattkörning",
                    ),
                    (
                        "/overnight status".into(),
                        "Visa status för senaste nattkörningen",
                    ),
                    ("/overnight log".into(), "Visa senaste nattkörningshändelser"),
                    ("/overnight review".into(), "Öppna den genererade granskningssidan"),
                    ("/overnight cancel".into(), "Begär avbrott av nattkörningen"),
                ],
            );
        }

        if prefix.starts_with("/subscription ") {
            return self.rank_suggestions(
                input,
                vec![("/subscription status".into(), "Visa prenumerationsstatus")],
            );
        }

        if prefix.starts_with("/alignment ") {
            return self.rank_suggestions(
                input,
                vec![
                    (
                        "/alignment status".into(),
                        "Visa aktuell och sparad justering",
                    ),
                    (
                        "/alignment centered".into(),
                        "Spara centrerad justering och tillämpa direkt",
                    ),
                    (
                        "/alignment left".into(),
                        "Spara vänsterjusterad layout och tillämpa direkt",
                    ),
                ],
            );
        }

        if prefix.starts_with("/compact-notifications ") {
            return self.rank_suggestions(
                input,
                vec![
                    (
                        "/compact-notifications status".into(),
                        "Visa om notiser är kompakta",
                    ),
                    (
                        "/compact-notifications on".into(),
                        "Fäll ihop swarm- och filnotiser till en rad",
                    ),
                    (
                        "/compact-notifications off".into(),
                        "Visa fullständiga flerradiga notiskort",
                    ),
                ],
            );
        }

        if prefix.starts_with("/tool-call-details ") {
            return self.rank_suggestions(
                input,
                vec![
                    (
                        "/tool-call-details status".into(),
                        "Visa om tekniska detaljer ritas på avsiktsrader",
                    ),
                    (
                        "/tool-call-details on".into(),
                        "Visa den dämpade tekniska detaljen intill verktygsavsikter",
                    ),
                    (
                        "/tool-call-details off".into(),
                        "Visa bara avsikten på verktygsrader som har en",
                    ),
                ],
            );
        }

        if prefix.starts_with("/show-agentgrep-output ") {
            return self.rank_suggestions(
                input,
                vec![
                    (
                        "/show-agentgrep-output status".into(),
                        "Visa om agentgrep-utdata visas inbäddat",
                    ),
                    (
                        "/show-agentgrep-output on".into(),
                        "Visa fullständiga agentgrep-träffar inbäddat i chatten",
                    ),
                    (
                        "/show-agentgrep-output off".into(),
                        "Visa bara agentgreps enradiga sammanfattning",
                    ),
                ],
            );
        }

        if prefix.starts_with("/config ") {
            return self.rank_suggestions(
                input,
                vec![
                    ("/config init".into(), "Skapa en standardkonfigurationsfil"),
                    ("/config create".into(), "Alias för /config init"),
                    ("/config edit".into(), "Öppna konfigurationsfilen i $EDITOR"),
                ],
            );
        }

        if prefix.starts_with("/goals show ") {
            let relevant_goals = crate::goal::list_relevant_goals(
                self.session
                    .working_dir
                    .as_deref()
                    .map(std::path::Path::new),
            )
            .unwrap_or_default();
            let suggestions = relevant_goals
                .into_iter()
                .map(|goal| (format!("/goals show {}", goal.id), "Open this goal"))
                .collect();
            return self.rank_suggestions(input, suggestions);
        }

        if prefix.starts_with("/goals ") {
            return self.rank_suggestions(
                input,
                vec![
                    ("/goals resume".into(), "Återuppta aktuellt mål"),
                    ("/goals show".into(), "Öppna ett visst mål via id"),
                ],
            );
        }

        if prefix.starts_with("/selfdev ") {
            return self.rank_suggestions(
                input,
                vec![
                    (
                        "/selfdev status".into(),
                        "Visa aktuell status för self-dev och bygge",
                    ),
                    ("/selfdev enter".into(), "Öppna en tom self-dev-session"),
                    (
                        "/selfdev enter ".into(),
                        "Öppna en self-dev-session med en prompt",
                    ),
                ],
            );
        }

        if prefix.starts_with("/rewind ") {
            let arg = prefix.strip_prefix("/rewind ").unwrap_or_default().trim();
            let visible_count = self.session.rewind_target_count();

            // Rewind targets are 1-based visible conversation message numbers.
            // Do not fuzzy-rank numeric arguments: `/rewind 10` should never be
            // completed or preview-accepted as `/rewind 1` just because `1` is a
            // fuzzy prefix match. If a complete numeric target is present, only
            // surface the exact valid command.
            if !arg.is_empty() && arg.chars().all(|c| c.is_ascii_digit()) {
                if let Ok(n) = arg.parse::<usize>()
                    && (1..=visible_count).contains(&n)
                {
                    return vec![(format!("/rewind {}", n), "Rewind to this message")];
                }
                return Vec::new();
            }

            let suggestions = (1..=visible_count)
                .map(|n| (format!("/rewind {}", n), "Rewind to this message"))
                .collect();
            return self.rank_suggestions(input, suggestions);
        }

        self.rank_suggestions(&prefix, self.command_candidates())
    }

    /// Get command suggestions based on current input
    pub fn command_suggestions(&self) -> Vec<(String, &'static str)> {
        // Read up to eight times per frame; recomputing each time re-ranks
        // every registered command and skill (and can touch disk for some
        // prefixes). Memoize on the exact input plus the guard state the
        // branches below consult, so any transition still recomputes.
        let signature = self.command_suggestions_signature();
        let epoch = self.command_suggestions_epoch.get();
        if let Some(cache) = self.command_suggestions_cache.borrow().as_ref()
            && cache.epoch == epoch
            && cache.signature == signature
            && cache.input == self.input
        {
            return cache.suggestions.clone();
        }

        let suggestions = self.command_suggestions_uncached(&signature);
        *self.command_suggestions_cache.borrow_mut() = Some(CommandSuggestionsCache {
            input: self.input.clone(),
            signature,
            epoch,
            suggestions: suggestions.clone(),
        });
        suggestions
    }

    /// Advance the suggestion memo epoch, invalidating it. Called once per
    /// rendered frame so the memo only ever collapses reads *within* a frame
    /// and never serves data that predates a state change.
    pub(crate) fn advance_command_suggestions_epoch(&self) {
        self.command_suggestions_epoch
            .set(self.command_suggestions_epoch.get().wrapping_add(1));
    }

    /// Snapshot the non-input state that `command_suggestions` branches on
    /// before consulting the input buffer.
    pub(super) fn command_suggestions_signature(&self) -> CommandSuggestionsSignature {
        CommandSuggestionsSignature {
            pending_login: self.pending_login.is_some(),
            pending_account_input: self.pending_account_input.is_some(),
            pending_ssh_remote_name: self.pending_ssh_remote_name.is_some(),
            inline_preview_kind: self
                .inline_interactive_state
                .as_ref()
                .filter(|picker| picker.preview)
                .map(|picker| picker.kind),
        }
    }

    /// Uncached body of [`Self::command_suggestions`].
    pub(super) fn command_suggestions_uncached(
        &self,
        signature: &CommandSuggestionsSignature,
    ) -> Vec<(String, &'static str)> {
        // While an interactive prompt is waiting for typed input (API key,
        // OAuth callback, account label, SSH target), the composer is an
        // answer box, not a command line. Rendering the full command palette
        // there is misleading (issue #496): the only command those prompts
        // advertise is /cancel, so suggest exactly that and nothing else.
        if signature.pending_login
            || signature.pending_account_input
            || signature.pending_ssh_remote_name
        {
            let input = self.input.trim_start();
            let typed = input.trim_end();
            if !typed.is_empty() && typed.starts_with('/') && "/cancel".starts_with(typed) {
                return vec![("/cancel".into(), "Avbryt väntande prompt")];
            }
            return Vec::new();
        }

        // While an inline picker preview is open for the command being typed,
        // the picker itself is the suggestion surface. Rendering the textual
        // suggestion list underneath would duplicate it (and its rows are not
        // arrow-navigable anyway, since the preview claims Up/Down first).
        if let Some(kind) = signature.inline_preview_kind {
            let input = self.input.trim_start();
            let suppress = match kind {
                crate::tui::PickerKind::Model => {
                    input.starts_with("/model") || input.starts_with("/models")
                }
                crate::tui::PickerKind::Login => input.starts_with("/login"),
                _ => false,
            };
            if suppress {
                return Vec::new();
            }
        }
        self.get_suggestions_for(&self.input)
    }

    fn clamp_command_suggestion_selection(&mut self) -> Vec<(String, &'static str)> {
        let suggestions = self.command_suggestions();
        if suggestions.is_empty() {
            self.command_suggestion_selected = 0;
        } else {
            self.command_suggestion_selected = self
                .command_suggestion_selected
                .min(suggestions.len().saturating_sub(1));
        }
        suggestions
    }

    pub(super) fn move_command_suggestion_selection(&mut self, delta: i32) -> bool {
        let suggestions = self.clamp_command_suggestion_selection();
        if suggestions.is_empty() {
            return false;
        }

        let len = suggestions.len() as i32;
        let selected = self.command_suggestion_selected as i32;
        self.command_suggestion_selected = (selected + delta).rem_euclid(len) as usize;
        true
    }

    fn arrow_modifiers_allow_command_suggestion_navigation(modifiers: KeyModifiers) -> bool {
        !modifiers.intersects(
            KeyModifiers::CONTROL | KeyModifiers::ALT | KeyModifiers::SUPER | KeyModifiers::HYPER,
        )
    }

    pub(super) fn handle_command_suggestion_key(
        &mut self,
        code: KeyCode,
        modifiers: KeyModifiers,
    ) -> bool {
        if self.command_suggestions().is_empty() {
            return false;
        }

        match code {
            KeyCode::Down
                if Self::arrow_modifiers_allow_command_suggestion_navigation(modifiers) =>
            {
                self.move_command_suggestion_selection(1)
            }
            KeyCode::Up if Self::arrow_modifiers_allow_command_suggestion_navigation(modifiers) => {
                self.move_command_suggestion_selection(-1)
            }
            KeyCode::Char('j') if modifiers.contains(KeyModifiers::CONTROL) => {
                self.move_command_suggestion_selection(1)
            }
            KeyCode::Char('k') if modifiers.contains(KeyModifiers::CONTROL) => {
                self.move_command_suggestion_selection(-1)
            }
            KeyCode::Enter if modifiers.is_empty() => self.accept_selected_command_suggestion(),
            _ => false,
        }
    }

    pub(super) fn accept_selected_command_suggestion(&mut self) -> bool {
        let suggestions = self.clamp_command_suggestion_selection();
        let Some((cmd, _)) = suggestions.get(self.command_suggestion_selected).cloned() else {
            return false;
        };
        if cmd == self.input.trim() {
            return false;
        }

        self.remember_input_undo_state();
        self.input = cmd;
        self.cursor_pos = self.input.len();
        self.tab_completion_state = None;
        self.command_suggestion_selected = 0;
        self.sync_model_picker_preview_from_input();
        true
    }

    /// Whether to show the dedicated first-run onboarding welcome screen
    /// (gray telemetry header, prominent donut, welcome text, login prompt).
    ///
    /// This is true exactly when the empty screen is showing onboarding
    /// suggestion prompts (brand-new install / unauthenticated / new user) so
    /// the welcome layout and the suggestions stay in sync.
    pub fn onboarding_welcome_active(&self) -> bool {
        if self.onboarding_preview_mode {
            return true;
        }
        // While the guided onboarding flow is driving the pre-suggestion phases
        // (model select / continue prompt), keep the welcome screen up even
        // though the server may have pushed scaffolding messages. The flow
        // renders its own body via `onboarding_welcome_kind()`.
        if self.onboarding_flow_drives_welcome() {
            return true;
        }
        if !self.display_messages.is_empty() || self.is_processing {
            return false;
        }
        !self.suggestion_prompts().is_empty()
    }

    /// What the onboarding welcome screen should render in its body, driven by
    /// the active guided flow phase. Defaults to the starter suggestion cards.
    pub fn onboarding_welcome_kind(&self) -> crate::tui::OnboardingWelcomeKind {
        use crate::tui::OnboardingWelcomeKind;
        use crate::tui::app::onboarding_flow::{OnboardingPhase, SummaryPill, TelemetryLevel};
        match self.onboarding_phase() {
            Some(OnboardingPhase::Login { import }) => {
                let prompt = import.as_ref().map(|review| {
                    let rows = review
                        .candidates
                        .iter()
                        .enumerate()
                        .map(|(i, candidate)| crate::tui::LoginImportRow {
                            provider_summary: candidate.provider_summary().to_string(),
                            source_name: candidate.source_name().to_string(),
                            checked: review.checked.get(i).copied().unwrap_or(false),
                        })
                        .collect();
                    crate::tui::LoginImportPrompt {
                        rows,
                        cursor: review.cursor,
                        continue_focused: review.continue_focused,
                        choosing: review.choosing,
                        summary_pill: match review.summary_pill {
                            SummaryPill::Continue => crate::tui::ImportSummaryPill::Continue,
                            SummaryPill::Subscription => {
                                crate::tui::ImportSummaryPill::Subscription
                            }
                            SummaryPill::ImportLess => crate::tui::ImportSummaryPill::ImportLess,
                            SummaryPill::Telemetry => crate::tui::ImportSummaryPill::Telemetry,
                        },
                        telemetry: review.telemetry.map(|level| match level {
                            TelemetryLevel::Everything => crate::tui::TelemetryChoice::Everything,
                            TelemetryLevel::NoContent => crate::tui::TelemetryChoice::NoContent,
                            TelemetryLevel::Nothing => crate::tui::TelemetryChoice::Nothing,
                        }),
                        telemetry_env_forced_off: crate::telemetry::opt_out_forced_by_env(),
                        checked_count: review.checked_count(),
                        seconds_left: review.seconds_remaining(),
                    }
                });
                OnboardingWelcomeKind::Login {
                    import: prompt,
                    importing: self.onboarding_import_in_progress.is_some(),
                    error: self.onboarding_import_error.clone(),
                    // Only offer the agent-repair option on the failure screen,
                    // and only when we can name an agent the user recently used.
                    repair_agent_label: self.onboarding_import_error.as_ref().and_then(|_| {
                        crate::tui::app::onboarding_repair::detect_preferred_repair_agent()
                            .map(|a| a.label().to_string())
                    }),
                }
            }
            Some(OnboardingPhase::LoginOpenAi { yes_highlighted }) => {
                OnboardingWelcomeKind::LoginOpenAi {
                    yes_highlighted: *yes_highlighted,
                }
            }
            Some(OnboardingPhase::ModelSelect) => OnboardingWelcomeKind::Suggestions,
            Some(OnboardingPhase::ContinuePrompt {
                cli,
                yes_highlighted,
                shown_at,
            }) => {
                let total = crate::tui::app::onboarding_flow::DECISION_TIMEOUT.as_secs();
                let seconds_left = total.saturating_sub(shown_at.elapsed().as_secs());
                OnboardingWelcomeKind::ContinuePrompt {
                    cli_label: cli.label().to_string(),
                    yes_highlighted: *yes_highlighted,
                    seconds_left,
                }
            }
            _ => OnboardingWelcomeKind::Suggestions,
        }
    }

    /// Whether the guided onboarding flow is in a phase that should take over
    /// the welcome screen body (login, OpenAI-login prompt, or continue prompt).
    /// The transcript-pick phase uses the session-picker overlay instead, and
    /// the suggestions phase is the default welcome body.
    fn onboarding_flow_drives_welcome(&self) -> bool {
        use crate::tui::app::onboarding_flow::OnboardingPhase;
        matches!(
            self.onboarding_phase(),
            Some(OnboardingPhase::Login { .. })
                | Some(OnboardingPhase::LoginOpenAi { .. })
                | Some(OnboardingPhase::ContinuePrompt { .. })
        )
    }

    /// Get suggestion prompts for new users on the initial empty screen.
    /// Returns (label, prompt_text) pairs. Empty once user is experienced or not authenticated.
    pub fn suggestion_prompts(&self) -> Vec<(String, String)> {
        let preview_mode = self.onboarding_preview_mode;
        let is_canary = if self.is_remote {
            self.remote_is_canary.unwrap_or(self.session.is_canary)
        } else {
            self.session.is_canary
        };
        if is_canary && !preview_mode {
            return Vec::new();
        }

        let auth = crate::auth::AuthStatus::check_fast();
        if !auth.has_any_available() {
            return vec![("Log in to get started".to_string(), "/login".to_string())];
        }

        if (!self.display_messages.is_empty() || self.is_processing) && !preview_mode {
            return Vec::new();
        }

        let is_new_user = if preview_mode {
            true
        } else {
            Self::is_new_user_install()
        };

        if !is_new_user {
            return Vec::new();
        }

        let mut prompts = vec![
            (
                "Customize my terminal theme".to_string(),
                "Find what terminal I'm using, then change its background color to pitch black and make it slightly transparent. Apply the changes for me.".to_string(),
            ),
            (
                "Review something I've been working on".to_string(),
                "Find a recent file or project I've been working on, read through it, and give me concrete suggestions on how I could improve it.".to_string(),
            ),
        ];

        // macOS-only: offer to install ScrollWM, a scrolling window manager for
        // macOS. The web installer downloads the latest release, strips the
        // Gatekeeper quarantine, installs to ~/Applications, and launches it,
        // with no sudo and no system files touched.
        if cfg!(target_os = "macos") {
            prompts.push((
                "Install ScrollWM (scrolling window manager for macOS)".to_string(),
                "Install ScrollWM, the scrolling window manager for macOS, by running its official one-line installer: `curl -fsSL https://raw.githubusercontent.com/1jehuang/scrollwm/main/scripts/web-install.sh | bash`. It downloads the latest release, removes the Gatekeeper quarantine, installs to ~/Applications, and launches it (no sudo, no system files touched). Run the command for me and report whether it succeeded.".to_string(),
            ));
        }

        prompts.push((
            "Continue my last Codex CLI / Claude Code session".to_string(),
            latest_external_cli_continuation_prompt().unwrap_or_else(|| {
                "Find my recent Codex or Claude Code sessions, identify the latest useful one, summarize what was happening, and continue from there.".to_string()
            }),
        ));

        prompts.push((
            "Find my social media and roast me".to_string(),
            "Find a social media platform I use, look around at my profile and posts, then give me a brutally honest roast based on what you see.".to_string(),
        ));

        prompts
    }

    /// Autocomplete current input - cycles through suggestions on repeated Tab
    pub fn autocomplete(&mut self) -> bool {
        // Get suggestions for current input
        let current_suggestions = self.get_suggestions_for(&self.input);

        // Check if we're continuing a tab cycle from a previous base
        if let Some((ref base, idx)) = self.tab_completion_state.clone() {
            let base_suggestions = self.get_suggestions_for(base);

            // If current input is in base suggestions AND there are multiple options, continue cycling
            if base_suggestions.len() > 1
                && base_suggestions.iter().any(|(cmd, _)| cmd == &self.input)
            {
                let next_index = (idx + 1) % base_suggestions.len();
                let (cmd, _) = &base_suggestions[next_index];
                self.remember_input_undo_state();
                self.input = cmd.clone();
                self.cursor_pos = self.input.len();
                self.tab_completion_state = Some((base.clone(), next_index));
                return true;
            }
            // Otherwise, fall through to start a new cycle with current input
        }

        // Start fresh cycle with current input
        if current_suggestions.is_empty() {
            self.tab_completion_state = None;
            return false;
        }

        // If only one suggestion and it matches exactly, add trailing space for commands
        // that accept arguments, then we're done
        if current_suggestions.len() == 1 && current_suggestions[0].0 == self.input {
            if !self.input.ends_with(' ') && Self::command_accepts_args(&self.input) {
                self.remember_input_undo_state();
                self.input.push(' ');
                self.cursor_pos = self.input.len();
                return true;
            }
            self.tab_completion_state = None;
            return false;
        }

        // Apply first suggestion and start tracking the cycle
        let selected = self
            .command_suggestion_selected
            .min(current_suggestions.len().saturating_sub(1));
        let (cmd, _) = &current_suggestions[selected];
        let base = self.input.clone();
        self.remember_input_undo_state();
        self.input = cmd.clone();
        // If unique match, add trailing space for arg-accepting commands
        if current_suggestions.len() == 1 && Self::command_accepts_args(&self.input) {
            self.input.push(' ');
        }
        self.cursor_pos = self.input.len();
        self.tab_completion_state = Some((base, selected));
        self.command_suggestion_selected = 0;
        true
    }

    /// Reset tab completion state (call when user types/modifies input)
    pub fn reset_tab_completion(&mut self) {
        self.tab_completion_state = None;
        self.command_suggestion_selected = 0;
    }

    pub(super) fn remember_input_undo_state(&mut self) {
        let snapshot = (self.input.clone(), self.cursor_pos.min(self.input.len()));
        if self.input_undo_stack.last() == Some(&snapshot) {
            return;
        }
        if self.input_undo_stack.len() >= Self::INPUT_UNDO_LIMIT {
            self.input_undo_stack.remove(0);
        }
        self.input_undo_stack.push(snapshot);
    }

    pub(super) fn clear_input_undo_history(&mut self) {
        self.input_undo_stack.clear();
    }

    pub(super) fn undo_input_change(&mut self) {
        if let Some((input, cursor_pos)) = self.input_undo_stack.pop() {
            self.input = input;
            self.cursor_pos = cursor_pos.min(self.input.len());
            self.reset_tab_completion();
            self.sync_model_picker_preview_from_input();
            self.set_status_notice("↶ Input restored");
        } else {
            self.set_status_notice("Nothing to undo");
        }
    }

    pub(super) fn command_accepts_args(cmd: &str) -> bool {
        matches!(
            cmd.trim(),
            "/help"
                | "/?"
                | "/btw"
                | "/fork"
                | "/git"
                | "/transcript"
                | "/observe"
                | "/todos"
                | "/splitview"
                | "/split-view"
                | "/model"
                | "/agents"
                | "/effort"
                | "/fast"
                | "/transport"
                | "/login"
                | "/auth"
                | "/account"
                | "/account claude"
                | "/account switch"
                | "/account openai"
                | "/account openai-compatible"
                | "/account default-provider"
                | "/account default-model"
                | "/account claude switch"
                | "/account claude remove"
                | "/account openai switch"
                | "/account openai remove"
                | "/usage"
                | "/subscription"
                | "/poke"
                | "/memory"
                | "/test"
                | "/initiatives"
                | "/initiatives show"
                | "/goals"
                | "/goals show"
                | "/swarm"
                | "/plan"
                | "/improve"
                | "/refactor"
                | "/rewind"
                | "/compact"
                | "/compact mode"
                | "/alignment"
                | "/compact-notifications"
                | "/show-agentgrep-output"
                | "/reasoning"
                | "/thinking"
                | "/thinking-display"
                | "/config"
                | "/save"
                | "/rename"
                | "/cache"
        )
    }
}

#[derive(Clone, Debug)]
struct ExternalCliSuggestionCandidate {
    source: &'static str,
    path: PathBuf,
    modified: SystemTime,
    session_id: Option<String>,
    working_dir: Option<String>,
    context: Option<String>,
}

/// How long a scan of the external-CLI session directories is reused before we
/// re-scan. The onboarding welcome screen animates a donut, so it redraws at
/// animation FPS and calls [`latest_external_cli_continuation_prompt`] multiple
/// times per frame. Scanning `~/.codex/sessions` / `~/.claude/projects` (reading
/// and JSON-parsing the newest transcripts) can cost hundreds of milliseconds
/// for users with large histories, which would otherwise make first-run
/// onboarding extremely laggy. A short TTL keeps the suggestion fresh while
/// reducing the cost to a single scan per window.
const EXTERNAL_CLI_PROMPT_CACHE_TTL: std::time::Duration = std::time::Duration::from_secs(30);

/// Cached result of the external-CLI continuation-prompt scan, with the time it
/// was computed. `None` value means "scanned, but nothing found".
type ExternalCliPromptCache = std::sync::RwLock<Option<(Option<String>, std::time::Instant)>>;
static EXTERNAL_CLI_PROMPT_CACHE: std::sync::LazyLock<ExternalCliPromptCache> =
    std::sync::LazyLock::new(|| std::sync::RwLock::new(None));

/// Cached front-end for [`latest_external_cli_continuation_prompt_uncached`].
///
/// See [`EXTERNAL_CLI_PROMPT_CACHE_TTL`] for why this is cached: the uncached
/// scan reads and parses the newest external transcripts, which is expensive for
/// large histories and would otherwise run several times per onboarding frame.
fn latest_external_cli_continuation_prompt() -> Option<String> {
    if let Ok(cache) = EXTERNAL_CLI_PROMPT_CACHE.read()
        && let Some((ref value, ref when)) = *cache
        && when.elapsed() < EXTERNAL_CLI_PROMPT_CACHE_TTL
    {
        return value.clone();
    }

    let value = latest_external_cli_continuation_prompt_uncached();

    if let Ok(mut cache) = EXTERNAL_CLI_PROMPT_CACHE.write() {
        *cache = Some((value.clone(), std::time::Instant::now()));
    }

    value
}

fn latest_external_cli_continuation_prompt_uncached() -> Option<String> {
    let home = std::env::var_os("HOME").map(PathBuf::from)?;
    let mut candidates = Vec::new();
    candidates.extend(latest_jsonl_suggestion_candidates(
        &home.join(".codex/sessions"),
        "Codex",
        32,
    ));
    candidates.extend(latest_jsonl_suggestion_candidates(
        &home.join(".claude/projects"),
        "Claude Code",
        32,
    ));
    let candidate = candidates
        .into_iter()
        .max_by_key(|candidate| candidate.modified)?;
    let location = candidate
        .working_dir
        .as_deref()
        .map_or_else(String::new, |dir| {
            let label = Path::new(dir)
                .file_name()
                .and_then(|name| name.to_str())
                .filter(|name| !name.trim().is_empty())
                .unwrap_or(dir);
            format!(" in {label}")
        });
    let cwd = candidate
        .working_dir
        .as_deref()
        .map(|dir| format!(" cwd `{dir}`"))
        .unwrap_or_default();
    let session_id = candidate
        .session_id
        .as_deref()
        .map(|id| format!(" session `{id}`"))
        .unwrap_or_default();
    let context = candidate
        .context
        .as_deref()
        .map(|context| format!(": {}", compact_suggestion_text(context, 72)))
        .unwrap_or_default();
    Some(format!(
        "Continue the latest {source} session{location}. Transcript: `{path}`.{session_id}{cwd}{context}. Read that transcript if needed, summarize the current state, then continue from there.",
        source = candidate.source,
        path = candidate.path.display(),
    ))
}

fn latest_jsonl_suggestion_candidates(
    root: &Path,
    source: &'static str,
    scan_limit: usize,
) -> Vec<ExternalCliSuggestionCandidate> {
    if !root.is_dir() {
        return Vec::new();
    }
    let mut files = Vec::new();
    collect_jsonl_suggestion_files(root, &mut files);
    files.sort_by(|a, b| b.1.cmp(&a.1));
    files.truncate(scan_limit);
    files
        .into_iter()
        .filter_map(|(path, modified)| suggestion_candidate_from_jsonl(&path, source, modified))
        .collect()
}

fn collect_jsonl_suggestion_files(root: &Path, files: &mut Vec<(PathBuf, SystemTime)>) {
    let Ok(entries) = std::fs::read_dir(root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if metadata.is_dir() {
            collect_jsonl_suggestion_files(&path, files);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("jsonl") {
            files.push((path, metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH)));
        }
    }
}

fn suggestion_candidate_from_jsonl(
    path: &Path,
    source: &'static str,
    modified: SystemTime,
) -> Option<ExternalCliSuggestionCandidate> {
    let content = std::fs::read_to_string(path).ok()?;
    let mut working_dir = None;
    let mut session_id = None;
    let mut last_user_text = None;
    let mut summary_text = None;
    for line in content.lines() {
        let Ok(value) = serde_json::from_str::<serde_json::Value>(line.trim()) else {
            continue;
        };
        if working_dir.is_none() {
            working_dir = value
                .get("cwd")
                .or_else(|| value.get("payload").and_then(|payload| payload.get("cwd")))
                .and_then(|value| value.as_str())
                .map(str::to_string);
        }
        if session_id.is_none() {
            session_id = value
                .get("sessionId")
                .or_else(|| value.get("session_id"))
                .or_else(|| {
                    value
                        .get("payload")
                        .and_then(|payload| payload.get("session_id"))
                })
                .or_else(|| value.get("payload").and_then(|payload| payload.get("id")))
                .and_then(|value| value.as_str())
                .map(str::to_string);
        }
        if summary_text.is_none() {
            summary_text = value
                .get("summary")
                .or_else(|| value.get("lastPrompt"))
                .or_else(|| {
                    value
                        .get("payload")
                        .and_then(|payload| payload.get("summary"))
                })
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|text| !text.is_empty())
                .map(str::to_string);
        }
        if jsonl_suggestion_role(&value) == Some("user")
            && let Some(text) = jsonl_suggestion_text(&value)
            && !text.trim().is_empty()
        {
            last_user_text = Some(text);
        }
    }
    if working_dir.is_none()
        && session_id.is_none()
        && last_user_text.is_none()
        && summary_text.is_none()
    {
        return None;
    }
    Some(ExternalCliSuggestionCandidate {
        source,
        path: path.to_path_buf(),
        modified,
        session_id,
        working_dir,
        context: last_user_text.or(summary_text),
    })
}

fn jsonl_suggestion_role(value: &serde_json::Value) -> Option<&str> {
    value
        .get("message")
        .and_then(|message| message.get("role"))
        .or_else(|| value.get("role"))
        .or_else(|| value.get("payload").and_then(|payload| payload.get("role")))
        .or_else(|| value.get("type"))
        .and_then(|role| role.as_str())
}

fn jsonl_suggestion_text(value: &serde_json::Value) -> Option<String> {
    let content = value
        .get("message")
        .and_then(|message| message.get("content"))
        .or_else(|| value.get("lastPrompt"))
        .or_else(|| value.get("content"))
        .or_else(|| {
            value
                .get("payload")
                .and_then(|payload| payload.get("content"))
        })?;
    if let Some(text) = content
        .as_str()
        .map(str::trim)
        .filter(|text| !text.is_empty())
    {
        return Some(text.to_string());
    }
    let text = content
        .as_array()?
        .iter()
        .filter_map(|block| {
            block
                .get("text")
                .or_else(|| block.get("input_text"))
                .or_else(|| block.get("output_text"))
                .or_else(|| block.get("content"))
                .and_then(|value| value.as_str())
                .map(str::trim)
                .filter(|text| !text.is_empty())
        })
        .collect::<Vec<_>>()
        .join(" ");
    (!text.is_empty()).then_some(text)
}

fn compact_suggestion_text(text: &str, max_chars: usize) -> String {
    let compact = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if compact.chars().count() <= max_chars {
        return compact;
    }
    let mut truncated = compact
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect::<String>();
    truncated.push('…');
    truncated
}

#[cfg(test)]
mod external_cli_suggestion_tests {
    use super::*;
    use std::io::Write;

    /// Faithful, real-home measurement of the per-frame onboarding cost.
    /// Ignored by default (depends on local ~/.codex and ~/.claude contents).
    /// Run with:
    ///   cargo test -p jcode-tui --lib onboarding_suggestion_scan_cost -- --ignored --nocapture
    #[test]
    #[ignore]
    fn onboarding_suggestion_scan_cost() {
        use std::time::Instant;

        // Cold: the uncached scan that reads + JSON-parses the newest external
        // transcripts. This is the work that used to run several times per frame.
        let cold_start = Instant::now();
        let cold = latest_external_cli_continuation_prompt_uncached();
        let cold_ms = cold_start.elapsed().as_secs_f64() * 1000.0;

        // Warm: the cached front-end the onboarding screen actually calls. Prime
        // the cache once, then measure repeated calls (as a redrawing frame does).
        let _ = latest_external_cli_continuation_prompt();
        let runs = 1000;
        let warm_start = Instant::now();
        let mut warm = None;
        for _ in 0..runs {
            warm = latest_external_cli_continuation_prompt();
        }
        let warm_ms = warm_start.elapsed().as_secs_f64() * 1000.0 / runs as f64;

        eprintln!(
            "external-cli continuation prompt: cold(uncached)={cold_ms:.1} ms, \
             warm(cached, avg of {runs})={warm_ms:.4} ms; cold_some={}, warm_some={}",
            cold.is_some(),
            warm.is_some()
        );
    }

    #[test]
    fn parses_claude_code_jsonl_with_session_path_and_context() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("session.jsonl");
        std::fs::write(
            &path,
            r#"{"type":"queue-operation","operation":"enqueue","timestamp":"2026-05-28T02:30:54.188Z","sessionId":"abc","content":"queued prompt"}
{"type":"user","message":{"role":"user","content":"Organize my windows by project"},"cwd":"/home/jeremy","sessionId":"abc"}
{"type":"last-prompt","lastPrompt":"fallback prompt","sessionId":"abc"}
"#,
        )
        .expect("write fixture");

        let candidate =
            suggestion_candidate_from_jsonl(&path, "Claude Code", SystemTime::UNIX_EPOCH)
                .expect("candidate");
        assert_eq!(candidate.source, "Claude Code");
        assert_eq!(candidate.path, path);
        assert_eq!(candidate.session_id.as_deref(), Some("abc"));
        assert_eq!(candidate.working_dir.as_deref(), Some("/home/jeremy"));
        assert_eq!(
            candidate.context.as_deref(),
            Some("Organize my windows by project")
        );
    }

    #[test]
    fn parses_codex_input_text_blocks() {
        let temp = tempfile::tempdir().expect("tempdir");
        let path = temp.path().join("codex.jsonl");
        std::fs::write(
            &path,
            r#"{"type":"session_meta","payload":{"id":"sid","cwd":"/home/jeremy/jcode"}}
{"type":"response_item","payload":{"type":"message","role":"user","content":[{"type":"input_text","text":"check in on jcode"}]}}
"#,
        )
        .expect("write fixture");

        let candidate = suggestion_candidate_from_jsonl(&path, "Codex", SystemTime::UNIX_EPOCH)
            .expect("candidate");
        assert_eq!(candidate.session_id.as_deref(), Some("sid"));
        assert_eq!(candidate.working_dir.as_deref(), Some("/home/jeremy/jcode"));
        assert_eq!(candidate.context.as_deref(), Some("check in on jcode"));
    }

    #[test]
    fn discovery_sorts_after_collecting_nested_files() {
        let temp = tempfile::tempdir().expect("tempdir");
        let old_dir = temp.path().join("a");
        let new_dir = temp.path().join("z/deep");
        std::fs::create_dir_all(&old_dir).expect("old dir");
        std::fs::create_dir_all(&new_dir).expect("new dir");
        std::fs::write(
            old_dir.join("old.jsonl"),
            r#"{"type":"user","message":{"role":"user","content":"old"},"sessionId":"old"}"#,
        )
        .expect("old fixture");
        std::thread::sleep(std::time::Duration::from_millis(20));

        let new_path = new_dir.join("new.jsonl");
        std::fs::write(
            &new_path,
            r#"{"type":"user","message":{"role":"user","content":"new"},"sessionId":"new"}"#,
        )
        .expect("new fixture");
        // Ensure the newer file has a strictly later mtime even on coarse filesystems.
        let mut file = std::fs::OpenOptions::new()
            .append(true)
            .open(&new_path)
            .expect("open new");
        writeln!(file).expect("touch new");

        let candidates = latest_jsonl_suggestion_candidates(temp.path(), "Claude Code", 1);
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].context.as_deref(), Some("new"));
    }

    /// Every slash command must be registered exactly once. Duplicate entries
    /// mean two different handlers claim the same name, so which one runs
    /// depends on dispatch order rather than on the registry the palette and
    /// `/help` show the user.
    #[test]
    fn registered_commands_have_no_duplicate_names() {
        let mut seen = std::collections::HashSet::new();
        let duplicates: Vec<&str> = REGISTERED_COMMANDS
            .iter()
            .filter(|command| !seen.insert(command.name))
            .map(|command| command.name)
            .collect();
        assert!(
            duplicates.is_empty(),
            "duplicate slash command registrations: {:?}",
            duplicates
        );
    }

    /// Aliases users can actually type must be discoverable through the
    /// registry, otherwise autocomplete silently omits working commands.
    #[test]
    fn known_aliases_are_registered() {
        let names: std::collections::HashSet<&str> =
            REGISTERED_COMMANDS.iter().map(|c| c.name).collect();
        for alias in [
            "/keybindings",
            "/commit-and-push",
            "/resume-all",
            "/hotkeys",
            "/keys",
        ] {
            assert!(names.contains(alias), "{alias} is not registered");
        }
    }
}
