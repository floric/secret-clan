import App from "./App.svelte";
import css from "./main.css";

const app = new App({
  target: document.body,
  props: { name: "Main" },
});

export default app;
