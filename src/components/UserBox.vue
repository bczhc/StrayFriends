<script setup lang="ts">
let emit = defineEmits(['click']);

const props = defineProps<{
  avatarImage: string,
  username: string,
}>();
</script>

<template>
  <div id="username-div" @click="emit('userProfileClick')">
    <n-avatar :src="props.avatarImage" round/>

    <span id="username-text">{{ props.username }}</span>
  </div>
</template>

<style scoped lang="scss">
#username-div {
  padding-top: .5em;
  display: flex;
  align-items: center;
  cursor: pointer;
}

#username-div:hover {
  color: #36ad6a;
  transition: color 0.2s ease-in-out;
}

#username-div > *:nth-child(2) {
  margin-left: .5em;
}
</style>
