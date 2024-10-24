<script setup lang="ts">
let emit = defineEmits(['click']);

const props = defineProps<{
  avatarImage: string,
  username: string,
  mobileNumber?: string,
}>();
</script>

<template>
  <div id="username-div" @click="emit('userProfileClick')">
    <n-avatar :src="props.avatarImage" round/>

    <div id="username-text">
      <span>{{ props.username }}</span>
      <span v-if="props.mobileNumber">{{ props.mobileNumber }}</span>
    </div>
  </div>
</template>

<style scoped lang="scss">
#username-div {
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

#username-text {
  display: inline-flex;
  gap: 1em;
}
</style>
