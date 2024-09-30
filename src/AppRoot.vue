<script setup lang="ts">
</script>

<template>
  <n-message-provider>
    <n-dialog-provider>
      <n-modal-provider>
        <RouterView/>
      </n-modal-provider>
    </n-dialog-provider>
  </n-message-provider>
</template>

<style scoped lang="scss">
</style>
