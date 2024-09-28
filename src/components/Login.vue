<script setup lang="ts">
import {useMessage, useDialog} from 'naive-ui';
import HomeTitle from "./HomeTitle.vue";

let message = useMessage();
let dialog = useDialog();

function signupClick() {

}
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
        <n-button size="large" @click="signupClick">注册</n-button>
        <n-button size="large" @click="" type="primary">登录</n-button>
      </n-space>
    </n-form>
  </div>
</template>

<style scoped>
#panel {
  background-color: white;
  border-radius: 5px;
  padding: 20px;
  box-shadow: rgba(100, 100, 111, 0.2) 0 7px 29px 0;
}

#form > * {
  opacity: 1;
  background-color: white;
}
</style>
