# PC Dynamic Island

Une Dynamic Island pour Windows 10/11 : musique, ChatGPT, Claude et mode Double AI.

> État actuel : interface épurée + fenêtre flottante Tauri. La musique et les IA sont encore simulées (étapes suivantes).

## Obtenir le .exe (sans rien installer)

Le `.exe` est compilé automatiquement par GitHub sur un serveur Windows.

1. Crée un dépôt sur GitHub (ex. `pc-dynamic-island`), puis envoie-y ce dossier :
   ```bash
   git remote add origin https://github.com/TON-PSEUDO/pc-dynamic-island.git
   git branch -M main
   git push -u origin main
   ```
2. Lance la compilation, au choix :
   - **Test rapide** : onglet **Actions** → *Build Windows* → **Run workflow**. Le `.exe` est téléchargeable dans les *Artifacts* du run.
   - **Release publique** : crée un tag, GitHub publie l'installateur dans l'onglet **Releases** :
     ```bash
     git tag v0.1.0
     git push origin v0.1.0
     ```
3. Télécharge `PC Dynamic Island_0.1.0_x64-setup.exe` et installe-le.

## Compiler en local (optionnel, sur Windows)

Prérequis : Node.js 20+, Rust (rustup), WebView2 (déjà présent sur Windows 10/11).

```bash
npm install
npx tauri icon app-icon.png
npm run dev      # lancer en mode développement
npm run build    # produire l'installateur
```

## Aperçu dans le navigateur

Ouvre `prototype/index.html` : c'est exactement la même interface, sans les fonctions Windows.

## Raccourcis (locaux pour l'instant)

`Alt+Espace` ouvrir/fermer · `Ctrl+1` ChatGPT · `Ctrl+2` Claude · `Ctrl+3` Double AI · `Échap` fermer.
Les raccourcis globaux Windows arriveront avec l'étape musique/IA.

## Sécurité

Les clés API ne seront jamais dans le code : elles seront saisies dans les paramètres et stockées dans le gestionnaire d'identifiants Windows. Le fichier `.gitignore` exclut déjà `.env` et `*.key`.
