<script setup lang="ts">

</script>

<template>
  <div id="panel">
    <n-h5>登录</n-h5>
    <n-form id="form" size="large">
      <n-form-item label="邮箱">
        <n-input/>
      </n-form-item>
      <n-form-item label="密码">
        <n-input type="password"/>
      </n-form-item>
      <n-space justify="space-evenly" size="large">
        <n-button size="large">注册</n-button>
        <n-button size="large">登录</n-button>
      </n-space>
    </n-form>
  </div>
</template>

<style scoped>
#panel {
  background-color: white;
  border-radius: 5px;
  padding: 20px;
}

#form > * {
  opacity: 1;
  background-color: white;
}
</style>
