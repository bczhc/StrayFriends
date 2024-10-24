<script setup lang="ts">
import {ref} from "vue";
import {apiGet, User} from "../api.ts";
import {genderDisplay} from "../main.ts";

const props = defineProps<{
  userId: number,
}>();
let user = ref<User | null>(null);

apiGet(`/api/user/${props.userId}`).then(r => {
  user.value = r as User;
}).catch(e => {
  // ignored
});
</script>

<template>
  <!-- TODO: in-progress feedback -->
  <div v-if="user" id="root">
    <n-avatar :src="'/avatar-demo.jpg'" id="avatar"/>
    <n-h5 class="label">用户名：{{ user.name }}</n-h5>
    <n-h5 class="label">邮箱：{{ user.email }}</n-h5>
    <n-h5 class="label">性别：{{ genderDisplay(user) }}</n-h5>
    <n-h5 class="label">个人简介</n-h5>
    <div>{{ user.bio }}</div>
  </div>
</template>

<style scoped lang="scss">
#avatar {
  $size: 50px;
  width: $size;
  height: $size;
}

.label {
  margin: 0
}

#root {
  background-color: white;
  padding: 10px;
  border-radius: 10px;
  box-shadow: rgba(50, 50, 93, 0.25) 0 13px 27px -5px, rgba(0, 0, 0, 0.3) 0px 8px 16px -8px;
}
</style>
