<script setup lang="ts">
import $ from 'jquery';
import {computed, ref} from "vue";
import {ApiResponse, checkEmail, WWW_FORM_URLENCODED_HEADER} from "../main.ts";
import axios from "axios";
import {useMessage} from "naive-ui";

let message = useMessage();

window.onload = () => {
  // I can only make the button full size via this
  $('div:has(> .full-width-btn)').css('width', '100%');
}

let email = ref<string>('');
let password = ref<string>('');
let passwordConfirm = ref<string>('');
let name = ref<string>('');

let emailValidation = computed(() => {
  return checkEmail(email.value);
});

let passwordInputStatus = computed(() => {
  if (password.value === passwordConfirm.value) {
    return 'success';
  } else {
    return 'error';
  }
});

let inProgress = ref<boolean>(false);

function signupClick() {
  if (name.value === '' || !emailValidation.value || passwordInputStatus.value === 'error') {
    message.error('请检查输入');
    return;
  }


  inProgress.value = true;
  axios.post('/api/signup', {
    name: name.value,
    email: email.value,
    password: password.value,
  }, {
    headers: WWW_FORM_URLENCODED_HEADER,
  }).then((x) => {
    let r = ApiResponse.from(x.data);
    console.log(r);
    if (r.success()) {
      message.success(r.message | '注册成功');
    } else {
      message.error(r.messageOrEmpty());
    }
  }).catch(e => {
    message.error(e.toString());
  }).finally(() => {
    inProgress.value = false;
  })
}

</script>

<template>
  <div id="panel">
    <n-h5>
      注册
      <n-icon size="1.5em" class="tr-button">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
          <path
              d="M289.94 256l95-95A24 24 0 0 0 351 127l-95 95l-95-95a24 24 0 0 0-34 34l95 95l-95 95a24 24 0 1 0 34 34l95-95l95 95a24 24 0 0 0 34-34z"
              fill="currentColor"></path>
        </svg>
      </n-icon>
    </n-h5>
    <n-form id="form" size="large">
      <n-form-item label="昵称">
        <n-input v-model:value="name"/>
      </n-form-item>
      <n-form-item label="邮箱">
        <n-input v-model:value="email" :status="emailValidation ? 'success' : 'error'"/>
      </n-form-item>
      <n-form-item label="密码">
        <n-input type="password" v-model:value="password" :status="passwordInputStatus"/>
      </n-form-item>
      <n-form-item label="重复密码">
        <n-input type="password" v-model:value="passwordConfirm" :status="passwordInputStatus"/>
      </n-form-item>
      <n-space justify="space-evenly" size="large">
        <n-button block class="full-width-btn" size="large" @click="signupClick" type="primary"
                  :disabled="inProgress"
        >
          <div style="display: inline-block">
            <span v-if="!inProgress">注册</span>
            <n-spin size="small" v-else style="display: inline; filter: invert(100%) brightness(500%)"/>
          </div>
        </n-button>
      </n-space>
    </n-form>
  </div>
</template>

<style scoped lang="scss">
$panel_padding: 20px;

#panel {
  background-color: white;
  border-radius: 5px;
  padding: $panel_padding;
  box-shadow: rgba(100, 100, 111, 0.2) 0 7px 29px 0;
}

#form > * {
  opacity: 1;
  background-color: white;
}

div:has(> .full-width-btn) {
  width: 100% !important;
}

.tr-button {
  opacity: .5;
  transition: .3s;
  position: absolute;
  right: $panel_padding;
  border-radius: 5px;
}

.tr-button:hover {
  background-color: gray;
}

h5 {
  vertical-align: middle;
}
</style>
