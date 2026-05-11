import * as THREE from 'three';
import init, { Tetris } from '../../wasm/pkg/tetris_logic';

let tetris: Tetris;
let scene: THREE.Scene;
let camera: THREE.PerspectiveCamera;
let renderer: THREE.WebGLRenderer;
let cubes: THREE.Mesh[][];
let lastTimestamp = 0;
let dropInterval = 500; // ms per drop
let lastDropTime = 0;

const COLORS = [
    0x000000, // empty
    0x00ffff, // I cyan
    0xffff00, // O yellow
    0xaa66ff, // T purple
    0x00ff00, // S green
    0xff0000, // Z red
    0xffaa00, // L orange
    0x0000ff, // J blue
];

async function main() {
    await init();
    tetris = Tetris.new();
    initThree();
    createBoardVisuals();
    animate(0);
    setupControls();
    document.getElementById('reset')!.onclick = () => {
        tetris.reset();
        updateBoardVisuals();
        (document.getElementById('score')!).innerText = '0';
    };
}

function initThree() {
    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x111122);
    camera = new THREE.PerspectiveCamera(45, window.innerWidth / window.innerHeight, 0.1, 1000);
    camera.position.set(12, 15, 18);
    camera.lookAt(5, 10, 0);
    renderer = new THREE.WebGLRenderer({ canvas: document.getElementById('gameCanvas') as HTMLCanvasElement });
    renderer.setSize(window.innerWidth, window.innerHeight);
    
    // Lights
    const ambient = new THREE.AmbientLight(0x404060);
    scene.add(ambient);
    const dirLight = new THREE.DirectionalLight(0xffffff, 1);
    dirLight.position.set(5, 10, 7);
    scene.add(dirLight);
    const backLight = new THREE.PointLight(0x4466cc, 0.5);
    backLight.position.set(-2, 5, -5);
    scene.add(backLight);
    
    // Grid helper
    const gridHelper = new THREE.GridHelper(20, 20, 0x88aaff, 0x335588);
    gridHelper.position.y = -0.5;
    scene.add(gridHelper);
}

function createBoardVisuals() {
    cubes = [];
    const blockSize = 0.9;
    const spacing = 1.0;
    for (let y = 0; y < 20; y++) {
        cubes[y] = [];
        for (let x = 0; x < 10; x++) {
            const geometry = new THREE.BoxGeometry(blockSize, blockSize, blockSize);
            const material = new THREE.MeshStandardMaterial({ color: 0x222222, emissive: 0x111111 });
            const cube = new THREE.Mesh(geometry, material);
            cube.position.set(x - 4.5, y - 9.5, 0);
            scene.add(cube);
            cubes[y][x] = cube;
        }
    }
}

function updateBoardVisuals() {
    const board = tetris.get_board();
    for (let y = 0; y < 20; y++) {
        for (let x = 0; x < 10; x++) {
            const val = board[y * 10 + x];
            const color = COLORS[val % COLORS.length];
            (cubes[y][x].material as THREE.MeshStandardMaterial).color.setHex(color);
            // Add slight glow for current piece
            (cubes[y][x].material as THREE.MeshStandardMaterial).emissiveIntensity = val > 0 ? 0.2 : 0;
        }
    }
    document.getElementById('score')!.innerText = tetris.get_score().toString();
    if (tetris.is_game_over()) {
        alert('Game Over! Press New Game.');
    }
}

function setupControls() {
    window.addEventListener('keydown', (e) => {
        switch(e.key) {
            case 'ArrowLeft': tetris.move_left(); updateBoardVisuals(); break;
            case 'ArrowRight': tetris.move_right(); updateBoardVisuals(); break;
            case 'ArrowUp': tetris.rotate(); updateBoardVisuals(); break;
            case 'ArrowDown': tetris.tick(); updateBoardVisuals(); break;
            case ' ': case 'Space': tetris.drop(); updateBoardVisuals(); break;
        }
        // Prevent scrolling
        if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', ' ', 'Space'].includes(e.key)) {
            e.preventDefault();
        }
    });
}

function animate(now: number) {
    requestAnimationFrame(animate);
    const delta = Math.min(100, now - lastTimestamp);
    lastTimestamp = now;
    
    // Automatic falling based on time
    if (!tetris.is_game_over() && now - lastDropTime > dropInterval) {
        tetris.tick();
        updateBoardVisuals();
        lastDropTime = now;
    }
    
    // Slight camera orbit for effect
    const time = Date.now() * 0.001;
    camera.position.x = 12 + Math.sin(time * 0.1) * 0.5;
    camera.lookAt(5, 10, 0);
    
    renderer.render(scene, camera);
}

main();
