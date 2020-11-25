<script lang="typescript">
  import Dialog from "../../components/layout/Dialog.svelte";
  import DialogHeader from "../../components/headers/DialogHeader.svelte";
  import BackToStart from "../../components/navigation/backToStart.svelte";
</script>

<Dialog>
  <DialogHeader>404 :(</DialogHeader>
  <p>The requested page seems to be missing...</p>
  <BackToStart />
</Dialog>
