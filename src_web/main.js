import init, { ResumeShaderApp } from '../pkg/sam_shader_engine.js';

let app = null;
let lastTime = performance.now();
let frameCount = 0;
let fpsTimer = 0;
const fpsDisplay = document.getElementById('hud-fps');

async function main() {
  try {
    // 1. Initialize Rust WebAssembly module
    await init();
    console.log('✓ Rust WebAssembly Shader Engine initialized successfully');

    // 2. Instantiate ResumeShaderApp attached to #shader-canvas
    app = new ResumeShaderApp('shader-canvas');

    // 3. Setup HUD Preset Switcher
    setupPresetSwitcher();

    // 4. Setup HUD Sliders
    setupSliders();

    // 5. Setup Window and Pointer Event Listeners
    setupInputListeners();

    // 6. Setup Resume Interactive Features (Filter & Print)
    setupResumeInteractions();

    // 7. Start Render Loop
    requestAnimationFrame(renderLoop);
  } catch (err) {
    console.error('Failed to initialize Rust Shader Engine:', err);
  }
}

function setupPresetSwitcher() {
  const container = document.getElementById('hud-presets-container');
  const descEl = document.getElementById('hud-effect-desc');
  if (!container || !app) return;

  const effectsList = app.get_effects_list();
  container.innerHTML = '';

  effectsList.forEach((name, idx) => {
    const btn = document.createElement('button');
    btn.className = `preset-btn ${idx === app.get_active_index() ? 'active' : ''}`;
    btn.dataset.index = idx;
    btn.textContent = name;

    btn.addEventListener('click', () => {
      try {
        app.switch_effect(idx);
        container.querySelectorAll('.preset-btn').forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        if (descEl) {
          descEl.textContent = app.get_active_description();
        }
        syncSlidersFromEngine();
      } catch (e) {
        console.error('Error switching effect:', e);
      }
    });

    container.appendChild(btn);
  });

  if (descEl) {
    descEl.textContent = app.get_active_description();
  }
}

function setupSliders() {
  const speedSlider = document.getElementById('param-speed');
  const warpSlider = document.getElementById('param-warp');
  const glowSlider = document.getElementById('param-glow');
  const colorSlider = document.getElementById('param-color');

  const speedVal = document.getElementById('val-speed');
  const warpVal = document.getElementById('val-warp');
  const glowVal = document.getElementById('val-glow');
  const colorVal = document.getElementById('val-color');

  speedSlider?.addEventListener('input', (e) => {
    const val = parseFloat(e.target.value);
    app.set_speed(val);
    if (speedVal) speedVal.textContent = `${val.toFixed(1)}x`;
  });

  warpSlider?.addEventListener('input', (e) => {
    const val = parseFloat(e.target.value);
    app.set_warp(val);
    if (warpVal) warpVal.textContent = `${val.toFixed(1)}x`;
  });

  glowSlider?.addEventListener('input', (e) => {
    const val = parseFloat(e.target.value);
    app.set_glow(val);
    if (glowVal) glowVal.textContent = `${val.toFixed(1)}x`;
  });

  colorSlider?.addEventListener('input', (e) => {
    const val = parseFloat(e.target.value);
    app.set_color_shift(val);
    if (colorVal) colorVal.textContent = val.toFixed(2);
  });

  // Toggle HUD collapse
  const toggleBtn = document.getElementById('hud-toggle-btn');
  const hudPanel = document.getElementById('shader-hud');
  toggleBtn?.addEventListener('click', () => {
    hudPanel?.classList.toggle('collapsed');
  });

  syncSlidersFromEngine();
}

function syncSlidersFromEngine() {
  if (!app) return;
  const params = app.get_params(); // [speed, warp, glow, color_shift]

  const speedSlider = document.getElementById('param-speed');
  const warpSlider = document.getElementById('param-warp');
  const glowSlider = document.getElementById('param-glow');
  const colorSlider = document.getElementById('param-color');

  if (speedSlider) speedSlider.value = params[0];
  if (warpSlider) warpSlider.value = params[1];
  if (glowSlider) glowSlider.value = params[2];
  if (colorSlider) colorSlider.value = params[3];

  const speedVal = document.getElementById('val-speed');
  const warpVal = document.getElementById('val-warp');
  const glowVal = document.getElementById('val-glow');
  const colorVal = document.getElementById('val-color');

  if (speedVal) speedVal.textContent = `${params[0].toFixed(1)}x`;
  if (warpVal) warpVal.textContent = `${params[1].toFixed(1)}x`;
  if (glowVal) glowVal.textContent = `${params[2].toFixed(1)}x`;
  if (colorVal) colorVal.textContent = params[3].toFixed(2);
}

function setupInputListeners() {
  // Mouse and Touch Pointer movement
  window.addEventListener('pointermove', (e) => {
    if (app) {
      app.on_pointer_move(e.clientX, e.clientY);
    }
  }, { passive: true });

  // Mouse and Touch Pointer click for ripple shockwaves
  window.addEventListener('pointerdown', (e) => {
    // Ignore clicks inside HUD or interactive controls
    if (e.target.closest('#shader-hud') || e.target.closest('button') || e.target.closest('a') || e.target.closest('input')) {
      return;
    }
    if (app) {
      const ndcX = (e.clientX / window.innerWidth) * 2.0 - 1.0;
      const ndcY = 1.0 - (e.clientY / window.innerHeight) * 2.0;
      app.on_pointer_down(ndcX, ndcY);
    }
  });

  // Window scroll reaction
  window.addEventListener('scroll', () => {
    if (app) {
      app.on_scroll(window.scrollY);
    }
  }, { passive: true });

  // Window resize
  window.addEventListener('resize', () => {
    if (app) {
      app.resize();
    }
  });
}

function setupResumeInteractions() {
  // Print button
  const printBtn = document.getElementById('btn-print');
  printBtn?.addEventListener('click', () => {
    window.print();
  });

  // Filter Focus buttons
  const filterBtns = document.querySelectorAll('.filter-btn');
  const timelineItems = document.querySelectorAll('.timeline-item');
  const skillCards = document.querySelectorAll('.skill-category-card');

  filterBtns.forEach((btn) => {
    btn.addEventListener('click', () => {
      const filter = btn.dataset.filter;
      filterBtns.forEach(b => b.classList.remove('active'));
      btn.classList.add('active');

      // Filter timeline
      timelineItems.forEach((item) => {
        const domains = (item.dataset.domains || '').split(' ');
        if (filter === 'all' || domains.includes(filter)) {
          item.classList.remove('dimmed');
        } else {
          item.classList.add('dimmed');
        }
      });

      // Highlight corresponding skill card
      skillCards.forEach((card) => {
        const domain = card.dataset.domain;
        if (filter === 'all' || domain === filter) {
          card.style.borderColor = 'rgba(0, 229, 255, 0.4)';
          card.style.background = 'rgba(0, 229, 255, 0.05)';
        } else {
          card.style.borderColor = 'rgba(255, 255, 255, 0.06)';
          card.style.background = 'rgba(255, 255, 255, 0.02)';
        }
      });
    });
  });
}

function renderLoop(currentTime) {
  if (app) {
    try {
      app.render(currentTime);
    } catch (e) {
      console.error('Error during shader render:', e);
    }
  }

  // FPS calculation
  frameCount++;
  const delta = currentTime - lastTime;
  fpsTimer += delta;
  lastTime = currentTime;

  if (fpsTimer >= 500) {
    const fps = Math.round((frameCount * 1000) / fpsTimer);
    if (fpsDisplay) {
      fpsDisplay.textContent = `${fps} FPS`;
    }
    frameCount = 0;
    fpsTimer = 0;
  }

  requestAnimationFrame(renderLoop);
}

// Start application when DOM is ready
if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', main);
} else {
  main();
}
