// @ts-expect-error
import Hls from "hls.js/dist/hls.min.js";

var video = document.getElementById("video") as HTMLMediaElement;
// const videoUrl =
//   "https://devstreaming-cdn.apple.com/videos/streaming/examples/img_bipbop_adv_example_ts/master.m3u8";
// if (Hls.isSupported()) {
//   var hls = new Hls({ debug: true });
//   hls.loadSource(videoUrl);
//   hls.attachMedia(video);
//   hls.on(Hls.Events.MANIFEST_PARSED, () => video.play());
// } else if (video.canPlayType("application/vnd.apple.mpegurl")) {
//   video.src = videoUrl;
// }

video.addEventListener("canplay", () => {
  video.play();
});