<script setup lang="ts">

import UserProfile from "./UserProfile.vue";

const props = defineProps<{
  avatarImage: string,
  username: string,
  mobileNumber?: string,
  userId?: number,
}>();
</script>

<template>
  <n-popover trigger="click" raw>
    <template #trigger>
      <div id="username-div">
        <n-avatar :src="props.avatarImage" round/>

        <div id="username-text">
          <span>{{ props.username }}</span>
          <span v-if="props.mobileNumber">{{ props.mobileNumber }}</span>
        </div>
      </div>
    </template>

    <!-- popup content -->
    <div v-if="props.userId">
      <UserProfile :user-id="props.userId"/>
    </div>
  </n-popover>
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
