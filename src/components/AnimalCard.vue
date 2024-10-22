<script setup lang="ts">
import {ref} from "vue";

let username = ref('用户名');
</script>

<template>
  <div style="border: 1px solid; display: inline-block" id="parent">
    <div class="center-parent">
      <img src="/2.jpg" alt="image" style="max-width: 100%"/>
      <n-h2 class="label">流浪狗</n-h2>
    </div>
    <div style="padding: 5px">
      <n-text style="word-wrap: break-word">
        路边发现的流浪狗，，，，，，，，，，，，，，，，，，，，
      </n-text>
    </div>
    <div id="username-div">
      <n-avatar src="/avatar-demo.jpg" round/>
      <span id="username-text">{{ username }}</span>
    </div>
  </div>

</template>

<style scoped lang="scss">
.label {
  margin: 0;
  padding: 0;
}

.center-parent {
  text-align: center;
}

#parent {
  padding: .5em;
  width: 20em;
}

#username-div {
  padding-top: .5em;
  display: flex;
  align-items: center;
}

#username-div > *:nth-child(2) {
  margin-left: .5em;
}

</style>
