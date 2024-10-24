<script setup lang="ts">
import {useDialog, useMessage} from 'naive-ui';
import {ref} from "vue";
import SpinIndicator from "./SpinIndicator.vue";
import {apiGet, apiPost, useAxios} from "../api.ts";
import {delay, messageError} from "../main.ts";
import {useRoute, useRouter} from "vue-router";
import {JWT_GET, JWT_STORE} from "../jwt.ts";

let message = useMessage();
let dialog = useDialog();
let axios = useAxios();
let router = useRouter();
let route = useRoute();

let email = ref('');
let password = ref('');

let emit = defineEmits<{
  signup: [],
}>();

function signupClick() {
  emit('signup');
}

function loginClick() {
  loginSuccess.value = false;
  inProgress.value = true;
  apiPost('/api/login', {
    username: email.value,
    password: password.value,
  }).then(r => {
    inProgress.value = false;
    loginSuccess.value = true;
    let token = r as string;
    JWT_STORE(token);
    delay(1000).then(() => {
      router.push('/home');
    });
  }).catch(x => messageError(x, message)).finally(() => {
    inProgress.value = false;
  });
}

let loginSuccess = ref(false);
let inProgress = ref(false);

if (route.query['type'] === 'logout') {
  router.replace({query: null});
  message.success('已登出');
}

let token = JWT_GET();
// go to Home page if the token is validated
apiGet('/api/me').then(_r => {
  router.push('/home');
}).catch(_r => {
  // ignored
})
</script>

<template>
  <div id="panel">
    <n-h5>登录</n-h5>
    <n-form id="form" size="large">
      <n-form-item label="邮箱">
        <n-input v-model:value="email" v-on:keyup.enter="loginClick"/>
      </n-form-item>
      <n-form-item label="密码">
        <n-input type="password" v-model:value="password" v-on:keyup.enter="loginClick"/>
      </n-form-item>
      <n-space justify="space-evenly" size="large">
        <n-button size="large" @click="signupClick">注册</n-button>
        <n-button size="large" @click="loginClick" type="primary"
                  :disabled="inProgress"
        >
          <span v-if="!inProgress">登录</span>
          <SpinIndicator v-else/>
        </n-button>
      </n-space>
    </n-form>
    <n-alert type="success"
             title="登录成功"
             id="login-success-alert"
             v-if="loginSuccess"
    >
      正在为您跳转
    </n-alert>
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

#login-success-alert {
  margin-top: 1em;
}
</style>
