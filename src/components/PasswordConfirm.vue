<script setup lang="ts">
import {computed, watch} from "vue";

let password = defineModel('password');
let passwordConfirm = defineModel('passwordConfirm');
let emit = defineEmits(['validated']);

watch([password, passwordConfirm], () => {
  emit('validated', passwordInputStatus.value === 'success');
});

let passwordInputStatus = computed(() => {
  if (password.value === passwordConfirm.value) {
    return 'success';
  } else {
    return 'error';
  }
});
</script>

<template>
  <n-form-item label="新密码">
    <n-input type="password" v-model:value="password" :status="passwordInputStatus"/>
  </n-form-item>
  <n-form-item label="重复密码">
    <n-input type="password" v-model:value="passwordConfirm" :status="passwordInputStatus"/>
  </n-form-item>
</template>

<style scoped lang="scss">

</style>
