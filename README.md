# Sam Tindal — Interactive Resume SPA & Rust WebAssembly Shader Engine

A high-performance Single Page Application (SPA) displaying the professional resume of **Sam Tindal**, powered by an **Object-Oriented Rust WebAssembly (WebGL2) Shader Engine** rendering real-time raymarched effects.

Hosted at: **[samtindal.com](https://samtindal.com)**

---

## Key Highlights

- **Object-Oriented Rust Shader Architecture**:
  - `ShaderEffect` Trait with polymorphic effect registry.
  - Three real-time WebGL2 shader presets:
    1. **Quantum Core**: Raymarched 4D pulsating core with volumetric chromatic dispersion and gyroscopic mouse reaction.
    2. **Cyber Grid**: Infinite perspective synthwave grid with terrain elevation harmonics, horizon glow, and wave propagation.
    3. **Gravitational Nebula**: Fluid fractal Brownian motion (fBm) deep space nebula with gravitational mouse vortex.
  - Encapsulated controllers: `InputController` (pointer smoothing, ripple shockwaves, scroll parallax), `UniformManager` (dynamic caching and binding), and `CanvasManager` (Retina DPR resolution scaling).
- **Test-Driven Development (TDD)**:
  - 100% passing Rust automated unit tests verifying coordinate normalization, pointer smoothing, ripple shockwave lifespan decay, uniform parameter clamping, and polymorphic effect switching.
- **Complete Professional Resume Content**:
  - Full experience covering 9 years across cloud platforms (Amazon Kuiper, Whole Foods Market, Amazon Inc., AWS, and r4 Technologies).
  - Specialized focus in Security & Compliance (ITAR / HIPAA / GDPR, FIDO2 / WebAuthn air-gapped authentication architecture, IAM least-privilege, threat modeling).
  - Interactive domain filters (Security, Cloud, Infrastructure, Data).
- **Print & PDF Optimized**:
  - Dedicated `@media print` stylesheet that cleanly hides the canvas and HUD, formatting the resume into an executive 2-page print layout.
- **Cloudflare Ready**:
  - Ready for deployment to Cloudflare Pages with custom domain `samtindal.com`, complete with automated GitHub Actions CI/CD and security headers.

---

## Project Structure

```
├── Cargo.toml                    # Rust WASM crate configuration
├── tests/
│   └── engine_tests.rs           # Automated unit tests (TDD)
├── src/                          # Object-Oriented Rust Shader Engine
│   ├── lib.rs                    # wasm_bindgen exported API (ResumeShaderApp)
│   ├── engine/
│   │   ├── mod.rs                # Engine module exports
│   │   ├── core.rs               # ShaderEngine: render loop & WebGL2 context
│   │   ├── canvas.rs             # CanvasManager: DPR scaling & viewport
│   │   ├── input.rs              # InputController: smoothing & ripple shockwaves
│   │   ├── uniforms.rs           # UniformManager: uniform caching & parameter clamping
│   │   └── buffer.rs             # GeometryBuffer: full-screen quad VAO
│   └── effects/
│       ├── mod.rs                # ShaderEffect trait & EffectRegistry
│       ├── quantum_core.rs       # Raymarched Quantum Core effect
│       ├── cyber_grid.rs         # Infinite Cyber Grid effect
│       └── gravitational_nebula.rs # Gravitational Nebula effect
├── index.html                    # Semantic HTML5 shell with resume markup & HUD
├── src_web/
│   ├── main.js                   # Web entrypoint & WASM coordinator
│   └── styles/
│       ├── main.css              # Design system tokens & glassmorphism
│       ├── hud.css               # Floating shader controls & FPS counter
│       ├── resume.css            # Resume card layout, timeline & badges
│       └── print.css             # High-fidelity 2-page print layout
├── public/
│   └── _headers                  # Cloudflare Pages security & wasm MIME headers
├── package.json                  # Vite build toolchain + wasm-pack scripts
├── vite.config.js                # Vite bundler config
├── wrangler.toml                 # Cloudflare Pages configuration
└── .github/workflows/deploy.yml  # Automated GitHub Actions CI/CD to Cloudflare
```

---

## Local Development & Testing

### 1. Run Automated Rust Unit Tests
```bash
cargo test
```

### 2. Install NPM Dependencies
```bash
npm install
```

### 3. Build WASM Module
```bash
npm run build:wasm
```

### 4. Start Local Development Server
```bash
npm run dev
```
Open your browser at `http://localhost:3000` to interact with the SPA and shaders!

### 5. Production Build
```bash
npm run build
```
The compiled, minified bundle will be output to `dist/`, ready for static hosting.

---

## Uploading to Git & Hosting on Cloudflare Pages

### Step 1: Initialize Git Repository & Push
```bash
git init
git add .
git commit -m "feat: complete Object-Oriented Rust Shader Engine and resume SPA"
git branch -M main
git remote add origin git@github.com:samtindal/rust-shaders-cv-spa.git
git push -u origin main
```

### Step 2: Deploy to Cloudflare Pages

#### Option A: Cloudflare Dashboard (Recommended)
1. Log in to your [Cloudflare Dashboard](https://dash.cloudflare.com/).
2. Navigate to **Workers & Pages** > **Create application** > **Pages** > **Connect to Git**.
3. Select your `samtindal.com` repository.
4. Set the build settings:
   - **Framework preset**: None
   - **Build command**: `npx wasm-pack build --target web --release && npm run build`
   - **Build output directory**: `dist`
   - **Environment variables**:
     - Add `NODE_VERSION` = `20`
5. Click **Save and Deploy**.

#### Option B: Automated GitHub Actions
If you prefer deploying via the included `.github/workflows/deploy.yml`:
1. In your GitHub repository settings, navigate to **Settings** > **Secrets and variables** > **Actions**.
2. Add the following repository secrets:
   - `CLOUDFLARE_API_TOKEN`: A Cloudflare API token with Pages edit permissions.
   - `CLOUDFLARE_ACCOUNT_ID`: Your Cloudflare account ID.
3. Pushes to `main` will automatically build the Rust WASM module and deploy to Cloudflare Pages.

### Step 3: Connect Custom Domain `samtindal.com`
1. In the Cloudflare Pages project settings, go to the **Custom domains** tab.
2. Click **Set up a custom domain**.
3. Enter `samtindal.com` (and optionally `www.samtindal.com`).
4. Since your DNS is managed on Cloudflare, DNS records (CNAME) will be configured automatically with zero downtime and automatic SSL/TLS encryption.
