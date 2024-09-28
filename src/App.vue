<script setup lang="ts">
import HomeTitle from "./components/HomeTitle.vue";
import Login from "./components/Login.vue";
import Signup from "./components/Signup.vue";
</script>

<template>
  <n-message-provider>
    <n-dialog-provider>
      <div id="background"></div>
      <div id="foreground">
        <div class="center">
<!--          <HomeTitle/>-->
<!--          <Login/>-->
          <Signup/>
        </div>
      </div>
    </n-dialog-provider>
  </n-message-provider>
</template>

<style scoped lang="scss">
#background {
  background-image: url("/1.webp");
  background-repeat: no-repeat;
  background-size: cover;
  width: 100%;
  height: 100vh;
  filter: blur(8px);
  position: fixed;
  transform: scale(1.1);
  opacity: 0.8;
}

#foreground {
  position: fixed;
  width: 100%;
  height: 100vh;
}

.center {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}
</style>
