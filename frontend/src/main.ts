import * as THREE from "three";
// @ts-expect-error
import { MindARThree } from "mind-ar/dist/mindar-image-three.prod.js";

const mindarThree = new MindARThree({
  container: document.querySelector("#container"),
  // imageTargetSrc: "/src/darts.mind",
  imageTargetSrc:
    "https://cdn.jsdelivr.net/gh/hiukim/mind-ar-js@1.2.0/examples/image-tracking/assets/card-example/card.mind",
  maxTrack: 1,
  filterMinCF: 0.01,
  filterBeta: 100,
  missTorelance: 8,
});
const { renderer, scene, camera } = mindarThree;
const video = document.getElementById("video") as HTMLVideoElement;
const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;

const anchor = mindarThree.addAnchor(0);
// 映像を投影するテクスチャを作成
const texture = new THREE.CanvasTexture(canvas);
texture.minFilter = THREE.LinearFilter;
texture.magFilter = THREE.LinearFilter;
texture.format = THREE.RGBAFormat;
const material = new THREE.MeshBasicMaterial({
  map: texture,
  transparent: true,
  opacity: 0.8,
});
// 平面ジオメトリを作成して、テクスチャを貼り付ける
const geometry = new THREE.PlaneGeometry(2, 2);
const mesh = new THREE.Mesh(geometry, material);
anchor.group.add(mesh);

const start = async () => {
  video.play();
  await mindarThree.start();
  renderer.setAnimationLoop(() => {
    // iOS Safari でテクスチャが透過表示されない問題のworkaround
    // canvasに一度描画し直す
    ctx.clearRect(0, 0, ctx.canvas.width, ctx.canvas.height);
    ctx.drawImage(video, 0, 0, ctx.canvas.width, ctx.canvas.height);
    texture.needsUpdate = !video.paused;
    renderer.render(scene, camera);
  });
};
const startButton = document.querySelector("#startButton");
startButton!.addEventListener("click", () => {
  start();
});
const stopButton = document.querySelector("#stopButton");
stopButton!.addEventListener("click", () => {
  mindarThree.stop();
  mindarThree.renderer.setAnimationLoop(null);
});
