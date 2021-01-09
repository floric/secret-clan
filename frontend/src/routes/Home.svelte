<script lang="typescript">
  import { link } from "svelte-spa-router";
  import PrimaryButton from "../components/buttons/Primary.svelte";
  import DialogHeader from "../components/headers/DialogHeader.svelte";
  import Dialog from "../components/layout/Dialog.svelte";
</script>

<Dialog>
  <DialogHeader>Welcome</DialogHeader>
  <p class="mb-6">
    These pages were only created to demonstrate the technology and serve as a
    foundation.
  </p>
  <a href="/games" use:link>
    <PrimaryButton>Let's go</PrimaryButton>
  </a>
</Dialog>
