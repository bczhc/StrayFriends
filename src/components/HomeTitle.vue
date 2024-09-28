<script setup lang="ts">

</script>

<template>
  <n-h1 id="title">流浪动物救助站</n-h1>
  <n-p id="subtitle"><i>
    <n-gradient-text type="info" :gradient="{
      from: 'blue',
      to: '#ff00e9',
      deg: 90
    }" style="filter: brightness(70%)">
      领养、捐助，或分享你的想法
    </n-gradient-text>
  </i></n-p>
</template>

<style scoped lang="scss">
@import url('https://fonts.googleapis.com/css2?family=Ma+Shan+Zheng&display=swap');

#title {
  font-size: 5em;
  margin: 0;
  font-family: "Ma Shan Zheng", sans-serif;
}

#subtitle {
  margin-top: 0;
  padding-top: 0;
  font-weight: bold
}
</style>
