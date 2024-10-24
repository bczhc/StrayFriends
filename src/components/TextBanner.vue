<script setup lang="ts">
import {computed} from "vue";

const props = defineProps<{
  height: string,
  text: string,
  fontSize: string,
}>();

let cssHeight = computed(() => props.height);
let cssFontSize = computed(() => props.fontSize);
</script>

<template>
  <div id="banner">
    <n-text class="display-large">{{ props.text }}</n-text>
  </div>
</template>

<style scoped lang="scss">
.display-large {
  font-size: v-bind(fontSize);
  font-weight: lighter;
  color: white;
}

#banner {
  height: v-bind(cssHeight);
  background-color: #c1633d;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
