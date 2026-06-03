<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import * as THREE from "three";
  import { OrbitControls } from "three/examples/jsm/controls/OrbitControls.js";

  interface Location {
    name: string;
    lat?: number;
    lng?: number;
  }

  interface Actor {
    name: string;
    role?: string;
  }

  interface TimelineEvent {
    uuid?: string; // Optional to align with master schema
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

  // Interactivity
  let raycaster = new THREE.Raycaster();
  let mouse = new THREE.Vector2();
  let hoveredObject: THREE.Object3D | null = null;
  let tooltipText = $state("");
  let tooltipX = $state(0);
  let tooltipY = $state(0);
  let showTooltip = $state(false);

  // Coordinate mapping boundaries
  // Longitude (X): Andalusia (-15) to China/Nusantara (120)
  const MIN_LNG = -15;
  const MAX_LNG = 120;
  const MAP_WIDTH = 120; // 3D units

  // Latitude (Y): Demak (-10) to Konstantinopel (45)
  const MIN_LAT = -10;
  const MAX_LAT = 45;
  const MAP_HEIGHT = 80; // 3D units

  // Time (Z): Adam AS (-4000) to Modern (2000)
  const MIN_YEAR = -4000;
  const MAX_YEAR = 2026;
  const MAP_DEPTH = 80; // 3D units (Vertical height)

  // Map values to 3D space
  // In Three.js: X is horizontal, Y is vertical (time), Z is horizontal depth (latitude)
  function get3DCoords(lng: number, lat: number, year: number): THREE.Vector3 {
    const x =
      ((lng - MIN_LNG) / (MAX_LNG - MIN_LNG)) * MAP_WIDTH - MAP_WIDTH / 2;
    const z = -(
      ((lat - MIN_LAT) / (MAX_LAT - MIN_LAT)) * MAP_HEIGHT -
      MAP_HEIGHT / 2
    ); // Negate for latitude direction
    const y =
      ((year - MIN_YEAR) / (MAX_YEAR - MIN_YEAR)) * MAP_DEPTH - MAP_DEPTH / 2;
    return new THREE.Vector3(x, y, z);
  }

  // Fallback coords for locations
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

  function getCoordsForEvent(ev: TimelineEvent): THREE.Vector3 {
    let lat = 23.8859;
    let lng = 45.0792; // Default Jazirah Arab

    if (ev.locations && ev.locations.length > 0) {
      const locName = ev.locations[0].name.toLowerCase();
      const matchedKey = Object.keys(locationCoordsRegistry).find(
        (k) => locName.includes(k) || k.includes(locName),
      );
      if (matchedKey) {
        lat = locationCoordsRegistry[matchedKey].lat;
        lng = locationCoordsRegistry[matchedKey].lng;
      }
    }
    return get3DCoords(lng, lat, ev.yearSort);
  }

  onMount(() => {
    if (!container) return;

    const width = container.clientWidth;
    const height = container.clientHeight;

    // Scene
    scene = new THREE.Scene();
    scene.background = new THREE.Color("#120e0c"); // Iron-900 / Surface
    scene.fog = new THREE.FogExp2("#120e0c", 0.003);

    // Camera
    camera = new THREE.PerspectiveCamera(60, width / height, 0.1, 1000);
    camera.position.set(90, 80, 110);

    // Renderer
    renderer = new THREE.WebGLRenderer({ antialias: true });
    renderer.setSize(width, height);
    renderer.setPixelRatio(window.devicePixelRatio);
    container.appendChild(renderer.domElement);

    // Controls
    controls = new OrbitControls(camera, renderer.domElement);
    controls.enableDamping = true;
    controls.dampingFactor = 0.05;
    controls.maxPolarAngle = Math.PI / 2 + 0.1; // Limit under ground level
    controls.minDistance = 10; // Allow 4x closer zoom-in
    controls.maxDistance = 1000; // Allow a bit more zoom-out as well

    // Lights
    const ambientLight = new THREE.AmbientLight(0xffffff, 0.4);
    scene.add(ambientLight);

    const dirLight1 = new THREE.DirectionalLight(0xffffff, 0.8);
    dirLight1.position.set(50, 150, 50);
    scene.add(dirLight1);

    const pointLight = new THREE.PointLight("#e4891b", 1, 100);
    pointLight.position.set(0, 0, 0);
    scene.add(pointLight);

    // ─── GRIDS & PLATES (SPACE-TIME COORDINATES) ────────────────
    // 1. Bottom Geography Ground Grid
    const groundGrid = new THREE.GridHelper(160, 32, "#38302a", "#1c1815");
    groundGrid.position.y = -MAP_DEPTH / 2 - 5;
    scene.add(groundGrid);

    // 2. Space-Time Era Plates (Transparent floating sheets)
    const eras = [
      { name: "Era Klasik Kuno", year: -3000, color: "#e4891b" },
      { name: "Era Kenabian Awal", year: -1500, color: "#e4891b" },
      { name: "Era Kerasulan Akhir", year: 610, color: "#cbd5e1" }, // Silver
      { name: "Era Khilafah & Daulah", year: 1000, color: "#cbd5e1" }, // Silver
      { name: "Era Kolonial & Modern", year: 1900, color: "#3b3734" },
    ];

    eras.forEach((era) => {
      const eraHeight =
        ((era.year - MIN_YEAR) / (MAX_YEAR - MIN_YEAR)) * MAP_DEPTH -
        MAP_DEPTH / 2;

      // Floating grid helper for visual reference
      const eraGrid = new THREE.GridHelper(120, 12, era.color, "#38302a");
      eraGrid.position.y = eraHeight;
      (eraGrid.material as THREE.Material).transparent = true;
      (eraGrid.material as THREE.Material).opacity = 0.15;
      scene.add(eraGrid);
    });

    // ─── GENERATE EVENTS & ACTORS SPLINES ──────────────────────
    eventGroup = new THREE.Group();
    splineGroup = new THREE.Group();
    projectionLinesGroup = new THREE.Group();
    scene.add(eventGroup);
    scene.add(splineGroup);
    scene.add(projectionLinesGroup);

    // Meshes will be built reactively in $effect

    // ─── INTERACTIVE HOVER & SELECTION ──────────────────────────
    function onMouseMove(event: MouseEvent) {
      const rect = renderer.domElement.getBoundingClientRect();
      mouse.x = ((event.clientX - rect.left) / rect.width) * 2 - 1;
      mouse.y = -((event.clientY - rect.top) / rect.height) * 2 + 1;

      raycaster.setFromCamera(mouse, camera);
      const intersects = raycaster.intersectObjects(eventGroup.children);

      if (intersects.length > 0) {
        const object = intersects[0].object as THREE.Mesh;
        if (hoveredObject !== object) {
          if (hoveredObject) {
            // Restore color
            const originalData = hoveredObject.userData.data;
            (hoveredObject as any).material.color.set(
              originalData.tier === "canonical" ? "#e4891b" : "#e2e8f0",
            ); // Perak penyala
            (hoveredObject as any).material.emissive.set(
              originalData.tier === "canonical" ? "#9a7024" : "#94a3b8",
            );
          }

          hoveredObject = object;
          // Highlight color
          (object as any).material.color.set("#fbbf24"); // Gold penyala
          (object as any).material.emissive.set("#b45309");

          // Position Tooltip
          const data = object.userData.data;
          tooltipText = `${data.title} (${data.year})`;
          tooltipX = event.clientX + 15;
          tooltipY = event.clientY - 25;
          showTooltip = true;
          document.body.style.cursor = "pointer";
        }
      } else {
        if (hoveredObject) {
          const originalData = hoveredObject.userData.data;
          (hoveredObject as any).material.color.set(
            originalData.tier === "canonical" ? "#e4891b" : "#e2e8f0",
          );
          (hoveredObject as any).material.emissive.set(
            originalData.tier === "canonical" ? "#9a7024" : "#94a3b8",
          );
          hoveredObject = null;
        }
        showTooltip = false;
        document.body.style.cursor = "default";
      }
    }

    function onClick() {
      if (hoveredObject && onEventSelect) {
        onEventSelect(hoveredObject.userData.data);
      }
    }

    window.addEventListener("mousemove", onMouseMove);
    window.addEventListener("click", onClick);

    // ─── RENDER LOOP ────────────────────────────────────────────
    function animate() {
      animationFrameId = requestAnimationFrame(animate);
      controls.update();

      // Gentle floating animation for event nodes
      const time = Date.now() * 0.001;
      eventGroup?.children.forEach((mesh, index) => {
        mesh.rotation.y += 0.01;
        mesh.position.y += Math.sin(time + index) * 0.005; // Gentle float
      });

      renderer.render(scene, camera);
    }

    animate();

    // ─── RESIZE HANDLING ────────────────────────────────────────
    function handleResize() {
      if (!container) return;
      const w = container.clientWidth;
      const h = container.clientHeight;
      camera.aspect = w / h;
      camera.updateProjectionMatrix();
      renderer.setSize(w, h);
    }

    window.addEventListener("resize", handleResize);

    // Clean up
    return () => {
      cancelAnimationFrame(animationFrameId);
      window.removeEventListener("mousemove", onMouseMove);
      window.removeEventListener("click", onClick);
      window.removeEventListener("resize", handleResize);
      if (renderer && container) {
        container.removeChild(renderer.domElement);
      }
    };
  });

  // ─── REACTIVE GRAPH BUILDER ─────────────────────────────────
  let eventGroup: THREE.Group;
  let splineGroup: THREE.Group;
  let projectionLinesGroup: THREE.Group;

  function clearGroup(group: THREE.Group) {
    if (!group) return;
    group.children.forEach((child) => {
      if (child instanceof THREE.Mesh || child instanceof THREE.Line) {
        child.geometry.dispose();
        if (Array.isArray(child.material)) {
          child.material.forEach((m) => m.dispose());
        } else {
          child.material.dispose();
        }
      }
    });
    group.clear();
  }

  $effect(() => {
    // Only run if the groups have been created inside onMount
    if (!eventGroup || !splineGroup || !projectionLinesGroup) return;

    // Clear existing elements to prevent duplicates
    clearGroup(eventGroup);
    clearGroup(splineGroup);
    clearGroup(projectionLinesGroup);

    const eventNodes: Array<{
      ev: TimelineEvent;
      pos: THREE.Vector3;
      mesh: THREE.Mesh;
    }> = [];

    // Track how many events at each location to spread them out
    const locationCounts: Record<string, number> = {};

    // Create Event Nodes
    events.forEach((ev: TimelineEvent) => {
      const pos = getCoordsForEvent(ev);

      // Determine location key for clustering
      let locKey = "default";
      if (ev.locations && ev.locations.length > 0) {
        const locName = ev.locations[0].name.toLowerCase();
        const matchedKey = Object.keys(locationCoordsRegistry).find(
          (k) => locName.includes(k) || k.includes(locName),
        );
        locKey = matchedKey || locName;
      }

      const count = locationCounts[locKey] || 0;
      locationCounts[locKey] = count + 1;

      // Add spatial distance/jitter for events in the same location (Spread out in a circle)
      if (count > 0) {
        const radius = 3.5 + Math.floor(count / 6) * 1.5; // Radius expands if many events
        const angle = count * ((Math.PI * 2) / 6); // Hexagonal spread
        pos.x += Math.cos(angle) * radius;
        pos.z += Math.sin(angle) * radius;
      }

      // Node Geometry (Octahedron for events)
      const geometry = new THREE.OctahedronGeometry(2.2, 0);
      const material = new THREE.MeshStandardMaterial({
        color: ev.tier === "canonical" ? "#e4891b" : "#e2e8f0", // Perak penyala
        emissive: ev.tier === "canonical" ? "#9a7024" : "#94a3b8",
        roughness: 0.2,
        metalness: 0.8,
      });
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.copy(pos);
      mesh.userData = { isEvent: true, data: ev };
      eventGroup.add(mesh);

      eventNodes.push({ ev, pos, mesh });

      // Ground Projection Line (Dashed vertical line to ground grid)
      const points = [];
      points.push(new THREE.Vector3(pos.x, -MAP_DEPTH / 2 - 5, pos.z));
      points.push(pos);

      const lineGeom = new THREE.BufferGeometry().setFromPoints(points);
      const lineMat = new THREE.LineDashedMaterial({
        color: "#595551",
        dashSize: 2,
        gapSize: 2,
      });
      const projLine = new THREE.Line(lineGeom, lineMat);
      projLine.computeLineDistances();
      projectionLinesGroup.add(projLine);
    });

    // Create Actor Splines (neon spline tracing events)
    const actorTrajectories: Record<string, THREE.Vector3[]> = {};

    eventNodes.forEach((node) => {
      node.ev.actors?.forEach((actor) => {
        if (!actorTrajectories[actor.name]) {
          actorTrajectories[actor.name] = [];
        }
        actorTrajectories[actor.name].push(node.pos);
      });
    });

    // Sort spline points chronologically
    Object.keys(actorTrajectories).forEach((actorName) => {
      const points = actorTrajectories[actorName];
      if (points.length < 2) return;

      points.sort((a, b) => a.y - b.y);

      const curve = new THREE.CatmullRomCurve3(points);
      const curvePoints = curve.getPoints(50);
      const curveGeom = new THREE.BufferGeometry().setFromPoints(curvePoints);

      const curveMat = new THREE.LineBasicMaterial({
        color: "#fbbf24", // Gold penyala for splines
        linewidth: 2,
      });

      const spline = new THREE.Line(curveGeom, curveMat);
      splineGroup.add(spline);

      const latestPos = points[points.length - 1];
      const indicatorGeom = new THREE.SphereGeometry(0.8, 8, 8);
      const indicatorMat = new THREE.MeshBasicMaterial({ color: "#fbbf24" }); // Gold penyala
      const indicator = new THREE.Mesh(indicatorGeom, indicatorMat);
      indicator.position.copy(latestPos);
      splineGroup.add(indicator);
    });
  });
</script>

<div class="relative w-full h-full flex flex-col min-h-[450px]">
  <!-- WebGL Canvas Container -->
  <div
    bind:this={container}
    class="w-full flex-1 rounded-2xl overflow-hidden border border-border/10 shadow-2xl relative z-0"
  ></div>

  <!-- Unified Info Panel: Legend & Navigation -->
  <div
    class="absolute bottom-6 left-6 z-10 glass p-4 rounded-xl border border-border/15 flex flex-col gap-4 pointer-events-none shadow-[0_8px_30px_rgba(0,0,0,0.6)] backdrop-blur-md min-w-[240px]"
  >
    <!-- Legend Section -->
    <div class="flex flex-col gap-2.5">
      <span
        class="text-[9px] font-bold text-text-secondary uppercase tracking-widest block border-b border-border/10 pb-1.5"
      >
        Legend Space-Time Cube
      </span>
      <div class="flex items-center gap-2.5 text-[9px]">
        <span
          class="w-2.5 h-2.5 bg-gold-500 rounded-sm shadow-[0_0_6px_rgba(212,168,83,0.8)] inline-block"
        ></span>
        <span class="text-text-primary font-medium"
          >Peristiwa (X, Y=Lokasi, Z=Waktu)</span
        >
      </div>
      <div class="flex items-center gap-2.5 text-[9px]">
        <span
          class="w-3.5 h-0.5 bg-yellow-400 inline-block shadow-[0_0_5px_rgba(251,191,36,0.6)]"
        ></span>
        <span class="text-text-primary font-medium"
          >Jejak Trajektori Tokoh (Spline)</span
        >
      </div>
      <div class="flex items-center gap-2.5 text-[9px]">
        <span
          class="w-3.5 h-0.5 border-t border-dashed border-slate-500 inline-block"
        ></span>
        <span class="text-text-primary font-medium"
          >Proyeksi Geografis Bumi</span
        >
      </div>
    </div>

    <!-- Navigation Section -->
    <div class="flex flex-col gap-2 pt-2 border-t border-border/10">
      <span
        class="text-[9px] font-bold text-gold-400 uppercase tracking-widest block mb-0.5 flex items-center gap-1.5"
      >
        <span>💡</span> Kendali Navigasi 3D
      </span>
      <div
        class="grid grid-cols-2 gap-x-2 gap-y-1.5 text-[8px] text-text-muted"
      >
        <span class="flex items-center gap-1.5"
          ><span class="bg-iron-900 px-1 py-0.5 rounded text-text-primary"
            >Drag Kiri</span
          > Putar Orbit</span
        >
        <span class="flex items-center gap-1.5"
          ><span class="bg-iron-900 px-1 py-0.5 rounded text-text-primary"
            >Drag Kanan</span
          > Geser (Pan)</span
        >
        <span class="flex items-center gap-1.5"
          ><span class="bg-iron-900 px-1 py-0.5 rounded text-text-primary"
            >Scroll</span
          > Zoom In/Out</span
        >
        <span class="flex items-center gap-1.5"
          ><span class="bg-iron-900 px-1 py-0.5 rounded text-text-primary"
            >Hover</span
          > Detail Node</span
        >
      </div>
    </div>
  </div>

  <!-- Realtime 3D Tooltip -->
  {#if showTooltip}
    <div
      class="fixed z-50 pointer-events-none px-3 py-1.5 rounded bg-iron-950/95 border border-verdigris-500/40 text-text-primary text-[10px] font-bold shadow-[0_4px_12px_rgba(0,0,0,0.5)] transition-all animate-fade-in"
      style="left: {tooltipX}px; top: {tooltipY}px;"
    >
      📌 {tooltipText}
    </div>
  {/if}
</div>
