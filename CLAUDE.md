# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

WikiGen is a Tauri-based desktop application that allows Pokemon ROM hack creators to generate user-friendly MkDocs wikis for documenting their game changes. The app manages Pokemon data, routes, trainers, moves, abilities, and more, then generates markdown files that are deployed to GitHub Pages.

**Tech Stack:**
- **Frontend:** SvelteKit + TypeScript, Tailwind CSS, bits-ui components
- **Backend:** Rust with Tauri 2.0
- **Database:** SQLite (one database per wiki)
- **Wiki Generation:** MkDocs with Material theme (Python)
- **Deployment:** Git + GitHub Pages

## Development Commands
```bash
# Start the server and open the desktop application
pnpm tauri dev

# Build frontend for production
pnpm tauri build
```

### Testing
- **Backend tests:** Located in `src-tauri/src/tests/`
- Test modules: `pokemon_generation_tests.rs`, `route_generation_tests.rs`, `migration_tests.rs`, etc.
- Run with `cargo test` from `src-tauri/` directory

## Architecture Overview

### High-Level Structure

WikiGen is a **Tauri application** combining a Rust backend with a SvelteKit frontend. Each wiki project is stored locally in `~/.app_data/WikiGen/[wiki_name]/` with its own SQLite database and generated markdown files.

**Data Flow:**
```
SvelteKit Frontend ←→ Tauri IPC Commands ←→ Rust Backend
                           ↓
                    SQLite Database
                           ↓
                    MkDocs Generation
                           ↓
                    Git + GitHub Pages
```

### Directory Structure

```
src/                           # SvelteKit frontend
├── routes/                    # File-based routing
│   ├── +layout.svelte        # Main navigation, wiki selector, state management
│   ├── pokemon/              # Pokemon editor page
│   ├── game-routes/          # Route/trainer editor
│   │   └── [slug]/           # Dynamic route detail pages
│   ├── moves/                # Move documentation
│   ├── attributes/           # Abilities, items, natures
│   └── deployment/           # GitHub deployment UI
├── lib/
│   ├── components/           # Reusable Svelte components
│   │   ├── ui/              # UI primitives (button, dialog, card, etc.)
│   │   └── modals/          # Modal components
│   ├── store/               # Svelte stores (reactive state)
│   │   ├── index.ts         # Main stores (wikis, selectedWiki, user)
│   │   ├── pokemon.ts       # Pokemon data types & store
│   │   ├── gameRoutes.ts    # Routes/trainers data
│   │   └── db.ts            # SQLite connection store
│   └── utils/               # Utilities (loadWiki, generators, etc.)

src-tauri/src/                # Rust backend
├── main.rs                   # Entry point, command registration
├── database/
│   └── mod.rs               # SQLite operations, file I/O
├── structs/                 # Data models
│   ├── pokemon_structs.rs   # DBPokemon, PokemonMove, etc.
│   └── mkdocs_structs.rs    # MkDocs config structures
├── page_generators/         # Markdown generation
│   ├── pokemon_pages.rs     # Generate Pokemon markdown
│   └── game_routes.rs       # Generate route markdown
├── wiki_preparation/        # Wiki lifecycle
│   ├── create_wiki.rs       # Initialize new wiki
│   ├── yaml_declaration.rs  # Update mkdocs.yml navigation
│   └── commit_wiki_changes.rs # Git operations
├── migrations/              # Database schema management
│   └── pokemon_migrations.rs
├── helpers/                 # Utilities
│   └── mkdocs_process.rs    # Spawn/kill MkDocs server
└── tests/                   # Rust tests
```

### Key Backend Modules

**main.rs:** Registers all Tauri commands that frontend can invoke via `invoke()`. Key commands:
- `create_wiki()` - Initialize new wiki project with SQLite DB and MkDocs template
- `generate_pokemon_pages_from_list()` - Create Pokemon markdown pages
- `generate_route_pages_with_handle()` - Create route markdown pages
- `commit_wiki_changes()` - Git commit and push to GitHub
- `spawn_mkdocs_process()` / `kill_mkdocs_process()` - Manage MkDocs dev server
- `update_yaml()` - Update mkdocs.yml navigation structure

**database/mod.rs:** Data access layer
- `get_sqlite_connection()` - Opens SQLite connection for specific wiki
- `get_mkdocs_config()` - Parses mkdocs.yml from disk
- `create_docs_file()` / `remove_docs_file()` - Markdown file operations
- `update_mkdocs_yaml()` - Write updated navigation to mkdocs.yml

**page_generators/:** Converts structured data to markdown pages
- `pokemon_pages.rs` - Creates formatted Pokemon pages with stats, evolution, movesets
- `game_routes.rs` - Creates route pages with wild encounters and trainer battles
- Updates mkdocs.yml navigation when pages are generated

**wiki_preparation/:** Project lifecycle management
- `create_wiki.rs` - Sets up directory structure, copies template, initializes DB
- `commit_wiki_changes.rs` - Git operations for deployment to GitHub Pages

**migrations/:** Database schema versioning
- Tracks which migrations have run per wiki
- Executes on app startup if new migrations detected

### Frontend Architecture

**Routing:** SvelteKit file-based routing in `src/routes/`
- `+layout.svelte` - Root layout with navigation, wiki selector, global state
- Each route corresponds to a feature page (pokemon, game-routes, deployment, etc.)

**State Management:** Svelte stores for reactive state
- `wikis` - All wiki metadata
- `selectedWiki` - Currently active wiki
- `pokemonList`, `itemsList`, `moveList` - Loaded from SQLite
- `Routes` - Route/trainer/wild encounter data from routes.json
- `db` - Active SQLite connection

**Data Loading:** When wiki is selected in `+layout.svelte`:
1. Opens SQLite connection via `Database.load()`
2. Loads Pokemon, moves, abilities, items from database
3. Loads routes.json and types.json from filesystem
4. Populates stores for reactive UI

**Component Patterns:**
- UI primitives from bits-ui (headless components)
- Custom components for features (modals, process spawn UI)
- Tailwind CSS with custom config for Pokemon type colors
- Toast notifications via svelte-sonner

### Data Storage

Each wiki is stored in `~/.app_data/WikiGen/[wiki_name]/`:
```
[wiki_name]/
├── [wiki_name].db           # SQLite database
├── dist/                    # Generated wiki
│   ├── mkdocs.yml          # Navigation config
│   ├── docs/               # Markdown files
│   │   ├── pokemon/        # 001-bulbasaur.md, etc.
│   │   ├── routes/         # route-1.md, etc.
│   │   └── img/pokemon/    # Sprite images
│   └── .git/               # Git repo for deployment
└── data/
    ├── routes.json         # Route/trainer/wild encounter data
    └── types.json          # Type effectiveness data
```

### Tauri IPC Pattern

Frontend calls backend via Tauri commands:
```typescript
// Frontend
await invoke("generate_pokemon_pages_from_list", {
  wikiName: "my-wiki",
  pokemonList: [1, 2, 3]
});
```

Backend command decorated with `#[tauri::command]`:
```rust
#[tauri::command]
fn generate_pokemon_pages_from_list(
    app_handle: AppHandle,
    wiki_name: String,
    pokemon_list: Vec<i32>
) -> Result<(), String> {
    // Implementation
}
```

All backend commands return `Result<T, String>` for error handling. Errors are logged and returned to frontend for display via toasts.

### Naming Conventions

**Backend (Rust):**
- snake_case for functions and variables
- Commands with `AppHandle` parameter suffixed with `_with_handle`
- Descriptive struct names: `DBPokemon`, `RouteProperties`, `WildEncounter`

**Frontend (TypeScript/Svelte):**
- camelCase for variables and functions
- PascalCase for components and types
- Store names lowercase (wikis, selectedWiki)
- Event handlers prefixed with `on` or descriptive of action

## Common Workflows

### Adding a New Tauri Command

1. Define command in appropriate module (e.g., `src-tauri/src/database/mod.rs`)
2. Add `#[tauri::command]` decorator
3. Register in `main.rs` `.invoke_handler()` list
4. Call from frontend: `await invoke("command_name", { args })`

### Creating New Page Generators

1. Add generator function in `src-tauri/src/page_generators/`
2. Generate markdown content from data structures
3. Use `create_docs_file()` to write markdown
4. Update mkdocs.yml navigation via `update_yaml()` or `update_mkdocs_yaml()`

### Adding Database Migrations

1. Create migration in `src-tauri/src/migrations/`
2. Define `Migration` struct with name, version, order, SQL
3. Add to migration list in `migrations/mod.rs`
4. Migration runs automatically on app startup

### Frontend Store Updates

1. Define store in `src/lib/store/[feature].ts`
2. Export writable store with initial state
3. Import and use in components: `import { storeName } from '$lib/store'`
4. Access reactively: `$storeName` or programmatically: `get(storeName)`

## Prerequisites for Development

1. **Node.js & pnpm** - Frontend package management
2. **Rust** - Backend compilation (install via rustup)
3. **Python 3** - Required for MkDocs wiki generation
4. **MkDocs & Material theme:**
   ```bash
   pip install mkdocs mkdocs-material
   ```

## Build Output

- **Development:** Frontend runs on http://localhost:5173, hot reload enabled
- **Production:** `pnpm run build` outputs to `/build`, then Tauri bundles into native app
- **App Binary:** Built apps located in `src-tauri/target/release/bundle/`

## WikiGen-Auth Companion Service

WikiGen uses a separate authentication service (**WikiGen-Auth**) to handle GitHub OAuth and repository creation. This service is located in `../WikiGen-Auth/` (sibling directory).

### Overview

**Tech Stack:**
- **Runtime:** Deno 2.0.4 (TypeScript)
- **Framework:** Hono (lightweight web framework)
- **Database:** Turso (LibSQL - edge SQLite)
- **JWT:** @panva/jose for token creation/verification
- **Deployment:** Fly.io at https://wikigen-auth.fly.dev

### API Endpoints

**GET `/authorize`**
- GitHub OAuth2 callback endpoint
- Exchanges authorization code for GitHub access token
- Stores user data (username, github_id, access_token) in Turso database
- Generates JWT token (8-hour expiration)
- Returns HTML page that invokes Tauri command to load token into desktop app

**POST `/create-repo`**
- Creates GitHub repository for wiki deployment
- Request body: `{ token: "jwt_token", wikiName: "wiki-name" }`
- Verifies JWT, retrieves GitHub access token from database
- Calls GitHub API to create repository
- Returns SSH URL: `git@github.com:username/wikiname.git`

### Authentication Flow

```
Desktop App → GitHub OAuth → WikiGen-Auth → Desktop App
```

1. User clicks "Sign In to GitHub" in desktop app
2. Opens webview to GitHub OAuth authorize URL (client_id: Ov23li9oWejO62cA6Kee)
3. User authorizes → GitHub redirects to WikiGen-Auth `/authorize`
4. WikiGen-Auth exchanges code for access token, stores in database
5. Returns `code.html` which calls Tauri command: `window.__TAURI__.core.invoke("load_token", { token })`
6. Desktop app receives JWT, stores in local `store.json`
7. JWT contains: user_name, avatar, expiration (HS256 algorithm)

### Repository Creation Flow

1. User initiates deployment in desktop app
2. Desktop app fetches JWT from local store
3. POST to WikiGen-Auth `/create-repo` endpoint
4. WikiGen-Auth validates JWT, retrieves GitHub access token
5. Creates GitHub repository via GitHub API
6. Returns SSH URL to desktop app
7. Desktop app stores SSH URL in wiki config for Git operations

### Development Commands (WikiGen-Auth)

```bash
# Navigate to WikiGen-Auth directory
cd ../WikiGen-Auth

# Run development server
deno task dev

# Run with specific permissions
deno run --allow-net --allow-read --allow-env server.ts

# Deploy to Fly.io
fly deploy
```

### Database Schema

**User Table (Turso):**
```sql
CREATE TABLE User (
  id INTEGER PRIMARY KEY,
  username TEXT,
  github_id INTEGER,
  access_token TEXT
)
```

### Key Files

- `server.ts` - Main application with all endpoints
- `tokenUtils.ts` - JWT creation and verification
- `code.html` - Response page that delivers token to Tauri app
- `fly.toml` - Fly.io deployment config
- `.env` - GitHub OAuth credentials, JWT secret, Turso credentials

### Important Notes

- JWT tokens expire after 8 hours
- Desktop app handles 401 responses by prompting re-login
- GitHub scopes: `read:user`, `public_repo`
- CORS enabled for all origins (accepts desktop app requests)
- Access tokens stored server-side for security (never exposed to desktop app)

## Deployment Flow

1. User signs in with GitHub OAuth (handled by WikiGen-Auth service)
2. WikiGen-Auth stores GitHub access token and returns JWT to desktop app
3. User clicks "Create Repository" in deployment page
4. Desktop app calls WikiGen-Auth `/create-repo` endpoint with JWT
5. WikiGen-Auth creates GitHub repository and returns SSH URL
6. User clicks "Deploy Wiki" in deployment page
7. Desktop app backend calls `commit_wiki_changes()`:
   - Initializes git in `dist/` folder
   - Commits all markdown files
   - Adds GitHub remote (SSH URL from step 5)
8. User manually pushes to GitHub (or uses GitHub Actions)
9. GitHub Pages serves the MkDocs site

## Important Notes

- Each wiki has its own isolated SQLite database
- Pokemon data comes preloaded with all 1025 Pokemon (embedded in resources)
- Routes/trainers/wild encounters stored in routes.json (not in SQLite)
- MkDocs dev server runs as spawned process managed by backend
- Tauri plugins used: fs, os, process, sql, shell, store, updater
- Auto-updater configured with pubkey and gist endpoint
