<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import * as THREE from "three";
  import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";

  interface Location { name: string; lat?: number; lng?: number; }
  interface Actor { name: string; role?: string; }
  interface TimelineEvent {
    uuid?: string;
    title: string;
    year: string;
    yearSort: number;
    description: string;
    tier: string;
    actors?: Actor[];
    locations?: Location[];
  }

  let { events = [], onEventSelect = null } = $props<{
    events: TimelineEvent[];
    onEventSelect?: ((event: any) => void) | null;
  }>();

  let container: HTMLDivElement | undefined = $state();
  let scene: THREE.Scene;
  let camera: THREE.PerspectiveCamera;
  let renderer: THREE.WebGLRenderer;
  let controls: OrbitControls;
  let animationFrameId: number;

  let raycaster = new THREE.Raycaster();
  let mouse = new THREE.Vector2();
  let hoveredObject: THREE.Object3D | null = null;
  let tooltipText = $state("");
  let tooltipX = $state(0);
  let tooltipY = $state(0);
  let showTooltip = $state(false);

  // Coordinate mapping
  const MIN_LNG = -15; const MAX_LNG = 120; const MAP_WIDTH = 120;
  const MIN_LAT = -10; const MAX_LAT = 45; const MAP_HEIGHT = 80;
  const MIN_YEAR = -4000; const MAX_YEAR = 2026; const MAP_DEPTH = 80;

  function get3DCoords(lng: number, lat: number, year: number): THREE.Vector3 {
    const x = ((lng - MIN_LNG) / (MAX_LNG - MIN_LNG)) * MAP_WIDTH - MAP_WIDTH / 2;
    const z = -(((lat - MIN_LAT) / (MAX_LAT - MIN_LAT)) * MAP_HEIGHT - MAP_HEIGHT / 2);
    const y = ((year - MIN_YEAR) / (MAX_YEAR - MIN_YEAR)) * MAP_DEPTH - MAP_DEPTH / 2;
    return new THREE.Vector3(x, y, z);
  }

  const locationCoordsRegistry: Record<string, { lat: number; lng: number }> = {
    makkah: { lat: 21.4225, lng: 39.8262 },
    madinah: { lat: 24.4672, lng: 39.6112 },
    "baitul maqdis": { lat: 31.778, lng: 35.2354 },
    baghdad: { lat: 33.3152, lng: 44.3661 },
    konstantinopel: { lat: 41.0082, lng: 28.9784 },
    damaskus: { lat: 33.5138, lng: 36.2765 },
    cordoba: { lat: 37.8882, lng: -4.7794 },
    demak: { lat: -6.8946, lng: 110.6385 },
    "samudera pasai": { lat: 5.1432, lng: 97.2341 },
    "jazirah arab": { lat: 23.8859, lng: 45.0792 },
    mesopotamia: { lat: 32.5, lng: 44.0 },
    alexandria: { lat: 31.2001, lng: 29.9187 },
    badr: { lat: 23.7744, lng: 38.7903 },
    "chang'an": { lat: 34.267, lng: 108.95 },
  };

  function getLocKey(ev: TimelineEvent): string | null {
    if (!ev.locations || ev.locations.length === 0) return null;
    const locName = ev.locations[0].name.toLowerCase();
    return Object.keys(locationCoordsRegistry).find(k => locName.includes(k) || k.includes(locName)) || null;
  }

  function getCoordsForEvent(ev: TimelineEvent): THREE.Vector3 {
    let lat = 23.8859, lng = 45.0792;
    const key = getLocKey(ev);
    if (key) { lat = locationCoordsRegistry[key].lat; lng = locationCoordsRegistry[key].lng; }
    return get3DCoords(lng, lat, ev.yearSort);
  }

  // ── Text Sprite Helper ──
  function makeTextSprite(text: string, opts: { fontSize?: number; color?: string; bgColor?: string; opacity?: number } = {}): THREE.Sprite {
    const fontSize = opts.fontSize || 48;
    const canvas = document.createElement("canvas");
    const ctx = canvas.getContext("2d")!;
    ctx.font = `bold ${fontSize}px Inter, Arial, sans-serif`;
    const metrics = ctx.measureText(text);
    const w = metrics.width + 24;
    const h = fontSize * 1.4;
    canvas.width = w; canvas.height = h;
    if (opts.bgColor) {
      ctx.fillStyle = opts.bgColor;
      ctx.globalAlpha = opts.opacity ?? 0.6;
      ctx.roundRect(0, 0, w, h, 6);
      ctx.fill();
      ctx.globalAlpha = 1;
    }
    ctx.font = `bold ${fontSize}px Inter, Arial, sans-serif`;
    ctx.fillStyle = opts.color || "#D4A853";
    ctx.textBaseline = "middle";
    ctx.textAlign = "center";
    ctx.fillText(text, w / 2, h / 2);
    const tex = new THREE.CanvasTexture(canvas);
    tex.minFilter = THREE.LinearFilter;
    const mat = new THREE.SpriteMaterial({ map: tex, transparent: true, depthTest: false });
    const sprite = new THREE.Sprite(mat);
    sprite.scale.set(w / 40, h / 40, 1);
    return sprite;
  }

  // ── Scene Groups ──
  let eventGroup: THREE.Group;
  let connectionGroup: THREE.Group;
  let labelGroup: THREE.Group;
  let locationPinGroup: THREE.Group;

  function clearGroup(group: THREE.Group) {
    if (!group) return;
    group.children.forEach(child => {
      if (child instanceof THREE.Mesh || child instanceof THREE.Line || child instanceof THREE.Points) {
        child.geometry?.dispose();
        if (Array.isArray(child.material)) child.material.forEach(m => m.dispose());
        else (child.material as THREE.Material)?.dispose();
      }
      if (child instanceof THREE.Sprite) (child.material as THREE.SpriteMaterial)?.map?.dispose();
    });
    group.clear();
  }

  onMount(() => {
    if (!container) return;
    const width = container.clientWidth;
    const height = container.clientHeight;

    scene = new THREE.Scene();
    scene.background = new THREE.Color("#08060a");
    scene.fog = new THREE.FogExp2("#08060a", 0.002);

    camera = new THREE.PerspectiveCamera(60, width / height, 0.1, 2000);
    camera.position.set(90, 80, 110);

    renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true });
    renderer.setSize(width, height);
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    container.appendChild(renderer.domElement);

    controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.05;
    controls.maxPolarAngle = Math.PI / 2 + 0.1;
    controls.minDistance = 10;
    controls.maxDistance = 1000;

    // ── Lights ──
    scene.add(new THREE.AmbientLight(0xffffff, 0.3));
    const dirLight = new THREE.DirectionalLight(0xffffff, 0.6);
    dirLight.position.set(50, 150, 50);
    scene.add(dirLight);
    const warmLight = new THREE.PointLight("#e4891b", 0.8, 200);
    warmLight.position.set(0, 20, 0);
    scene.add(warmLight);

    // ── 1. STARFIELD ──
    const starGeo = new THREE.BufferGeometry();
    const starPositions = new Float32Array(2000 * 3);
    const starColors = new Float32Array(2000 * 3);
    for (let i = 0; i < 2000; i++) {
      starPositions[i * 3] = (Math.random() - 0.5) * 800;
      starPositions[i * 3 + 1] = (Math.random() - 0.5) * 800;
      starPositions[i * 3 + 2] = (Math.random() - 0.5) * 800;
      const brightness = 0.3 + Math.random() * 0.7;
      const goldTint = Math.random() > 0.7;
      starColors[i * 3] = goldTint ? brightness : brightness * 0.8;
      starColors[i * 3 + 1] = goldTint ? brightness * 0.85 : brightness * 0.85;
      starColors[i * 3 + 2] = goldTint ? brightness * 0.5 : brightness;
    }
    starGeo.setAttribute("position", new THREE.BufferAttribute(starPositions, 3));
    starGeo.setAttribute("color", new THREE.BufferAttribute(starColors, 3));
    scene.add(new THREE.Points(starGeo, new THREE.PointsMaterial({ size: 0.6, vertexColors: true, transparent: true, opacity: 0.8 })));

    // ── 2. GROUND GRID ──
    const groundGrid = new THREE.GridHelper(160, 32, "#38302a", "#1c1815");
    groundGrid.position.y = -MAP_DEPTH / 2 - 5;
    scene.add(groundGrid);

    // ── 3. ERA PLANES + LABELS ──
    const eras = [
      { name: "Era Klasik Kuno", year: -3000, color: 0xe4891b },
      { name: "Era Kenabian Awal", year: -1500, color: 0xd4a853 },
      { name: "Era Kerasulan Akhir", year: 610, color: 0xcbd5e1 },
      { name: "Era Khilafah & Daulah", year: 1000, color: 0x5d8f8a },
      { name: "Era Modern", year: 1900, color: 0x3b3734 },
    ];
    eras.forEach(era => {
      const eraY = ((era.year - MIN_YEAR) / (MAX_YEAR - MIN_YEAR)) * MAP_DEPTH - MAP_DEPTH / 2;
      // Semi-transparent plane
      const planeGeo = new THREE.PlaneGeometry(140, 100);
      const planeMat = new THREE.MeshBasicMaterial({ color: era.color, transparent: true, opacity: 0.04, side: THREE.DoubleSide });
      const plane = new THREE.Mesh(planeGeo, planeMat);
      plane.rotation.x = -Math.PI / 2;
      plane.position.y = eraY;
      scene.add(plane);
      // Grid overlay
      const eraGrid = new THREE.GridHelper(120, 12, era.color, "#38302a");
      eraGrid.position.y = eraY;
      (eraGrid.material as THREE.Material).transparent = true;
      (eraGrid.material as THREE.Material).opacity = 0.12;
      scene.add(eraGrid);
      // Era text label
      const label = makeTextSprite(era.name, { fontSize: 32, color: `#${era.color.toString(16).padStart(6, '0')}`, bgColor: "#0a0a0a", opacity: 0.7 });
      label.position.set(-MAP_WIDTH / 2 - 8, eraY, 0);
      scene.add(label);
    });

    // ── 4. AXIS LABELS ──
    const yAxisLabel = makeTextSprite("WAKTU →", { fontSize: 36, color: "#D4A853" });
    yAxisLabel.position.set(-MAP_WIDTH / 2 - 12, 5, -MAP_HEIGHT / 2 - 5);
    scene.add(yAxisLabel);

    // Year markers along Y axis
    [-3000, -1500, 0, 622, 1000, 1453, 1900].forEach(yr => {
      const yPos = ((yr - MIN_YEAR) / (MAX_YEAR - MIN_YEAR)) * MAP_DEPTH - MAP_DEPTH / 2;
      const yearLabel = makeTextSprite(`${yr > 0 ? yr + ' M' : Math.abs(yr) + ' SM'}`, { fontSize: 24, color: "#94a3b8" });
      yearLabel.position.set(-MAP_WIDTH / 2 - 6, yPos, -MAP_HEIGHT / 2 - 3);
      scene.add(yearLabel);
    });

    // ── 5. GROUPS ──
    eventGroup = new THREE.Group();
    connectionGroup = new THREE.Group();
    labelGroup = new THREE.Group();
    locationPinGroup = new THREE.Group();
    scene.add(eventGroup, connectionGroup, labelGroup, locationPinGroup);

    // ── HOVER & CLICK ──
    function onMouseMove(event: MouseEvent) {
      const rect = renderer.domElement.getBoundingClientRect();
      mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
      mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;
      raycaster.setFromCamera(mouse, camera);
      const intersects = raycaster.intersectObjects(eventGroup.children);
      if (intersects.length > 0) {
        const object = intersects[0].object as THREE.Mesh;
        if (hoveredObject !== object) {
          if (hoveredObject) restoreNodeColor(hoveredObject);
          hoveredObject = object;
          (object.material as THREE.MeshStandardMaterial).color.set("#fbbf24");
          (object.material as THREE.MeshStandardMaterial).emissive.set("#b45309");
          const data = object.userData.data;
          tooltipText = `${data.title} (${data.year})`;
          tooltipX = event.clientX + 15;
          tooltipY = event.clientY - 25;
          showTooltip = true;
          document.body.style.cursor = "pointer";
        }
      } else {
        if (hoveredObject) { restoreNodeColor(hoveredObject); hoveredObject = null; }
        showTooltip = false;
        document.body.style.cursor = "default";
      }
    }

    function restoreNodeColor(obj: THREE.Object3D) {
      const d = obj.userData.data;
      const mat = (obj as THREE.Mesh).material as THREE.MeshStandardMaterial;
      mat.color.set(d.tier === "canonical" ? "#e4891b" : "#e2e8f0");
      mat.emissive.set(d.tier === "canonical" ? "#9a7024" : "#94a3b8");
    }

    function onClick() {
      if (hoveredObject && onEventSelect) onEventSelect(hoveredObject.userData.data);
    }

    window.addEventListener("mousemove", onMouseMove);
    window.addEventListener("click", onClick);

    // ── RENDER LOOP ──
    function animate() {
      animationFrameId = requestAnimationFrame(animate);
      controls.update();
      const time = Date.now() * 0.001;
      eventGroup?.children.forEach((mesh, i) => {
        mesh.rotation.y += 0.008;
        mesh.position.y += Math.sin(time + i * 0.5) * 0.003;
      });
      renderer.render(scene, camera);
    }
    animate();

    function handleResize() {
      if (!container) return;
      camera.aspect = container.clientWidth / container.clientHeight;
      camera.updateProjectionMatrix();
      renderer.setSize(container.clientWidth, container.clientHeight);
    }
    window.addEventListener("resize", handleResize);

    return () => {
      cancelAnimationFrame(animationFrameId);
      window.removeEventListener("mousemove", onMouseMove);
      window.removeEventListener("click", onClick);
      window.removeEventListener("resize", handleResize);
      if (renderer && container) container.removeChild(renderer.domElement);
    };
  });

  // ── REACTIVE GRAPH BUILDER ──
  $effect(() => {
    if (!eventGroup || !connectionGroup || !labelGroup || !locationPinGroup) return;
    clearGroup(eventGroup);
    clearGroup(connectionGroup);
    clearGroup(labelGroup);
    clearGroup(locationPinGroup);

    const overlapCounts: Record<string, number> = {};
    const eventNodes: Array<{ ev: TimelineEvent; pos: THREE.Vector3; mesh: THREE.Mesh }> = [];

    // ── A. Location Pins on Ground Plane ──
    const placedLocations = new Set<string>();
    const groundY = -MAP_DEPTH / 2 - 5;

    events.forEach((ev: TimelineEvent) => {
      const key = getLocKey(ev);
      if (key && !placedLocations.has(key)) {
        placedLocations.add(key);
        const coords = locationCoordsRegistry[key];
        const groundPos = get3DCoords(coords.lng, coords.lat, MIN_YEAR);
        // Pin (cone)
        const pinGeo = new THREE.ConeGeometry(1, 3, 6);
        const pinMat = new THREE.MeshStandardMaterial({ color: "#f59e0b", emissive: "#b45309", roughness: 0.3, metalness: 0.7 });
        const pin = new THREE.Mesh(pinGeo, pinMat);
        pin.position.set(groundPos.x, groundY + 1.5, groundPos.z);
        locationPinGroup.add(pin);
        // Glow ring
        const ringGeo = new THREE.RingGeometry(1.5, 2.2, 16);
        const ringMat = new THREE.MeshBasicMaterial({ color: "#f59e0b", transparent: true, opacity: 0.25, side: THREE.DoubleSide });
        const ring = new THREE.Mesh(ringGeo, ringMat);
        ring.rotation.x = -Math.PI / 2;
        ring.position.set(groundPos.x, groundY + 0.1, groundPos.z);
        locationPinGroup.add(ring);
        // Label
        const locLabel = makeTextSprite(ev.locations![0].name, { fontSize: 28, color: "#f59e0b", bgColor: "#0a0a0a", opacity: 0.6 });
        locLabel.position.set(groundPos.x, groundY + 5, groundPos.z);
        locationPinGroup.add(locLabel);
      }
    });

    // ── B. Event Nodes ──
    events.forEach((ev: TimelineEvent) => {
      const pos = getCoordsForEvent(ev);
      const locKey = getLocKey(ev) || "default";
      const overlapKey = `${locKey}_${ev.yearSort}`;
      const count = overlapCounts[overlapKey] || 0;
      overlapCounts[overlapKey] = count + 1;
      if (count > 0) {
        const radius = 2.5 + Math.floor(count / 6) * 1.5;
        const angle = count * ((Math.PI * 2) / 6);
        pos.x += Math.cos(angle) * radius;
        pos.z += Math.sin(angle) * radius;
      }

      // Variable node size based on actor count
      const actorCount = ev.actors?.length || 0;
      const nodeSize = 1.8 + Math.min(actorCount, 5) * 0.4;

      const geometry = new THREE.OctahedronGeometry(nodeSize, 0);
      const material = new THREE.MeshStandardMaterial({
        color: ev.tier === "canonical" ? "#e4891b" : "#e2e8f0",
        emissive: ev.tier === "canonical" ? "#9a7024" : "#94a3b8",
        emissiveIntensity: 0.6,
        roughness: 0.15,
        metalness: 0.85,
      });
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.copy(pos);
      mesh.userData = { isEvent: true, data: ev };
      eventGroup.add(mesh);

      // Subtle glow sprite behind each node
      const glowCanvas = document.createElement("canvas");
      glowCanvas.width = 64; glowCanvas.height = 64;
      const gCtx = glowCanvas.getContext("2d")!;
      const gradient = gCtx.createRadialGradient(32, 32, 0, 32, 32, 32);
      const glowColor = ev.tier === "canonical" ? "rgba(228,137,27," : "rgba(226,232,240,";
      gradient.addColorStop(0, glowColor + "0.4)");
      gradient.addColorStop(0.5, glowColor + "0.1)");
      gradient.addColorStop(1, glowColor + "0)");
      gCtx.fillStyle = gradient;
      gCtx.fillRect(0, 0, 64, 64);
      const glowTex = new THREE.CanvasTexture(glowCanvas);
      const glowSprite = new THREE.Sprite(new THREE.SpriteMaterial({ map: glowTex, transparent: true, depthWrite: false }));
      glowSprite.scale.set(nodeSize * 4, nodeSize * 4, 1);
      glowSprite.position.copy(pos);
      eventGroup.add(glowSprite);

      eventNodes.push({ ev, pos, mesh });
    });

    // ── C. Connection Lines ──
    // C1: By shared location (gold dashed)
    const locationGroups = new Map<string, THREE.Vector3[]>();
    eventNodes.forEach(({ ev, pos }) => {
      const key = getLocKey(ev);
      if (key) {
        if (!locationGroups.has(key)) locationGroups.set(key, []);
        locationGroups.get(key)!.push(pos.clone());
      }
    });
    locationGroups.forEach((positions) => {
      if (positions.length < 2) return;
      positions.sort((a, b) => a.y - b.y);
      const points = positions;
      const lineGeo = new THREE.BufferGeometry().setFromPoints(points);
      const lineMat = new THREE.LineBasicMaterial({ color: "#D4A853", transparent: true, opacity: 0.2 });
      connectionGroup.add(new THREE.Line(lineGeo, lineMat));
    });

    // C2: By shared actor (verdigris dotted)
    const actorGroups = new Map<string, THREE.Vector3[]>();
    eventNodes.forEach(({ ev, pos }) => {
      if (!ev.actors) return;
      ev.actors.forEach(a => {
        if (!actorGroups.has(a.name)) actorGroups.set(a.name, []);
        actorGroups.get(a.name)!.push(pos.clone());
      });
    });
    actorGroups.forEach((positions) => {
      if (positions.length < 2) return;
      positions.sort((a, b) => a.y - b.y);
      const lineGeo = new THREE.BufferGeometry().setFromPoints(positions);
      const lineMat = new THREE.LineBasicMaterial({ color: "#5d8f8a", transparent: true, opacity: 0.15 });
      connectionGroup.add(new THREE.Line(lineGeo, lineMat));
    });
  });
</script>

<div class="relative w-full h-full flex flex-col min-h-[450px]">
  <!-- WebGL Canvas -->
  <div bind:this={container} class="w-full flex-1 rounded-2xl overflow-hidden border border-border/10 shadow-2xl relative z-0"></div>

  <!-- Compact Legend (Bottom Center) -->
  <div class="absolute bottom-4 left-1/2 -translate-x-1/2 z-10 glass px-5 py-2.5 rounded-xl border border-border/15 flex items-center gap-5 pointer-events-none shadow-[0_8px_30px_rgba(0,0,0,0.6)] backdrop-blur-md">
    <div class="flex items-center gap-2 text-[9px]">
      <span class="w-3 h-3 bg-gold-500 rounded-sm shadow-[0_0_8px_rgba(212,168,83,0.8)]"></span>
      <span class="text-text-primary font-semibold">Peristiwa Canonical</span>
    </div>
    <div class="flex items-center gap-2 text-[9px]">
      <span class="w-3 h-3 bg-slate-200 rounded-sm shadow-[0_0_6px_rgba(226,232,240,0.5)]"></span>
      <span class="text-text-primary font-semibold">Peristiwa Reviewed</span>
    </div>
    <div class="flex items-center gap-2 text-[9px]">
      <span class="w-5 h-0.5 bg-gold-500/50"></span>
      <span class="text-text-muted">Lokasi</span>
    </div>
    <div class="flex items-center gap-2 text-[9px]">
      <span class="w-5 h-0.5 bg-verdigris-500/50"></span>
      <span class="text-text-muted">Tokoh</span>
    </div>
    <div class="flex items-center gap-2 text-[9px]">
      <span class="text-amber-400">📌</span>
      <span class="text-text-muted">Lokasi Geografis</span>
    </div>
  </div>

  <!-- Navigation Hints (Bottom Right, Minimal) -->
  <div class="absolute bottom-4 right-4 z-10 glass px-3 py-2 rounded-lg border border-border/10 pointer-events-none backdrop-blur-md">
    <div class="flex items-center gap-3 text-[8px] text-text-muted">
      <span><span class="text-text-primary font-bold">Drag</span> Putar</span>
      <span><span class="text-text-primary font-bold">Scroll</span> Zoom</span>
      <span><span class="text-text-primary font-bold">Klik</span> Detail</span>
    </div>
  </div>

  <!-- Tooltip -->
  {#if showTooltip}
    <div
      class="fixed z-50 pointer-events-none px-3 py-2 rounded-lg bg-iron-950/95 border border-gold-500/40 text-text-primary text-[11px] font-bold shadow-[0_4px_20px_rgba(0,0,0,0.7)] backdrop-blur-sm"
      style="left: {tooltipX}px; top: {tooltipY}px;"
    >
      📌 {tooltipText}
    </div>
  {/if}
</div>
