<script lang="typescript">
  import { link } from "svelte-spa-router";
  import Dialog from "../components/layout/Dialog.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import PrimaryButton from "../components/buttons/Primary.svelte";
</script>

<Dialog>
  <DialogHeader>Lobby</DialogHeader>
  <p>Hier ist viel zu tun:</p>
  <ul class="list-disc mb-6">
    <li>Prüfung auf gültiges Spiel und lande des zugehörigen Spieles</li>
    <li>Anzeige beigetretener Spieler</li>
    <li>Spieleinstellungen</li>
    <li>Fancy UI</li>
  </ul>
  <PrimaryButton>Start</PrimaryButton>
  <div class="flex items-center mt-4">
    <div class="flex ml-auto">
      <a
        href="/games"
        use:link
        class="inline-flex text-xs sm:text-sm text-blue-500 hover:text-blue-700">Back
        Games</a>
    </div>
  </div>
</Dialog>
