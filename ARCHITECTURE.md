# PC Dynamic Island – Architecture

## Choix de stack : Tauri 2 + interface HTML/CSS/JS légère (cœur Rust)

Pourquoi Tauri plutôt que WinUI 3 / WPF :
- RAM faible (~30-60 Mo, WebView2 déjà présent sur Win10/11), contre Electron (~200 Mo+).
- Animations fluides (CSS / spring) très proches du rendu Apple, plus faciles qu'en XAML.
- Rust donne un accès direct aux API Windows (crate `windows`) : médias, fenêtre, raccourcis.
- Installateur NSIS/MSI généré par `tauri build`.

## Modules

| Module | Techno | Rôle |
|---|---|---|
| Fenêtre île | Tauri window : sans bordure, transparente, always-on-top, `skip_taskbar`, centrée en haut, redimensionnée selon l'état | Barre flottante, multi-écrans (monitor sous le curseur) |
| Click-through | `set_ignore_cursor_events` hors de la zone de l'île | Ne gêne pas le PC |
| Médias | Rust + `windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager` (SMTC) | Titre, artiste, pochette, play/pause/suivant/précédent, position, événements |
| Volume | `IAudioEndpointVolume` (WASAPI) | Volume système |
| Notifications | `UserNotificationListener` (WinRT, demande d'autorisation) + événements internes | Notifications Windows, alertes, téléchargements |
| IA ChatGPT / Claude | Appels HTTP **côté Rust** (`reqwest`, streaming SSE) | Jamais de clé dans le front |
| Double AI | 2 requêtes parallèles (`tokio::join!`) + 3e requête de comparaison | Réponses côte à côte |
| Clés API | Windows Credential Manager (crate `keyring`) | Stockage sécurisé, jamais dans le code ni en clair |
| Limites / coûts | Compteur de tokens + plafond quotidien/mensuel + rate limiter local | Maîtrise des coûts |
| Historique | SQLite (`rusqlite`) local | Conversations, copie, reprise |
| Raccourcis | `tauri-plugin-global-shortcut` | Personnalisables |
| Paramètres | JSON (`tauri-plugin-store`) | Position, taille, couleur, blur, arrondi, vitesse, thème, modules |

## Structure

```
pc-dynamic-island/
├─ src-tauri/
│  ├─ src/
│  │  ├─ main.rs          # setup fenêtre, tray, raccourcis
│  │  ├─ media.rs         # SMTC
│  │  ├─ audio.rs         # volume
│  │  ├─ notifications.rs
│  │  ├─ ai/{openai.rs, anthropic.rs, double.rs, limits.rs}
│  │  ├─ secrets.rs       # keyring
│  │  └─ store.rs         # SQLite + settings
│  └─ tauri.conf.json
├─ ui/index.html          # interface (machine à états : idle | launcher | music | musicFull | chat | double | notif | settings)
├─ prototype/index.html   # copie pour aperçu dans le navigateur
├─ app-icon.png           # source des icônes (npx tauri icon)
└─ .github/workflows/build.yml  # compilation .exe + release GitHub
```

## États de l'île

`idle` (120×30) → `launcher` (290×56) → `music` (310×42) → `musicFull` (380×176) → `chat` (480×360) → `double` (860×400) → `notif` (temporaire) → `settings`.
Transition par morphing de largeur/hauteur/rayon avec courbe spring `cubic-bezier(.32,1.25,.4,1)`.

## Sécurité

- Clés saisies dans Paramètres → envoyées au Rust → stockées dans le Credential Manager.
- CSP stricte, front sans accès réseau direct vers les API IA.
- Rien n'est envoyé aux IA sans action explicite de l'utilisateur.
- Gestion d'erreurs : 401/429/5xx, timeouts, messages clairs dans l'île.

## Plan de livraison

1. Architecture ✅
2. Prototype visuel animé (ce document + `prototype/`)
3. Projet Tauri + SMTC (musique réelle)
4. ChatGPT, 5. Claude, 6. Double AI
7. Paramètres
8-9. Tests / corrections
10. `tauri build` → installateur `.exe`/`.msi`

> Choix : l'interface reste en HTML/CSS/JS pur (pas de React) : plus léger, aucun bundler, et le même fichier sert d'aperçu navigateur.

> Note : le build `.exe` et les tests SMTC doivent s'exécuter sur Windows (le workspace ici est Linux). Je prépare tout le code ; la compilation finale se fait sur ta machine ou via GitHub Actions (runner `windows-latest`).
