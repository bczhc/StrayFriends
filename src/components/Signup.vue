<script setup lang="ts">
import {computed, ref} from "vue";
import {apiPost} from "../api.ts";
import {useMessage} from "naive-ui";
import SpinIndicator from "./SpinIndicator.vue";
import PasswordConfirm from "./PasswordConfirm.vue";
import {checkEmail, messageError} from "../main.ts";

let emit = defineEmits<{
  back: [],
}>()

let message = useMessage();

let email = ref<string>('');
let password = ref<string>('');
let passwordConfirm = ref<string>('');
let name = ref<string>('');
let passwordValidated = ref(true);

let emailValidation = computed(() => {
  return checkEmail(email.value);
});

let inProgress = ref<boolean>(false);

function signupClick() {
  if (name.value === '' || !emailValidation.value || !passwordValidated) {
    message.error('请检查输入');
    return;
  }


  inProgress.value = true;
  apiPost('/api/signup', {
    name: name.value,
    email: email.value,
    password: password.value,
  }).then(_r => {
    message.success('注册成功');
    emit('back');
  }).catch(e => messageError(e, message)).finally(() => {
    inProgress.value = false;
  });
}

</script>

<template>
  <div id="panel">
    <n-h5>
      注册
      <n-icon size="1.5em" class="tr-button" @click="emit('back')">
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
      <PasswordConfirm
          v-model:password="password"
          v-model:password-confirm="passwordConfirm"
          @validated="x => passwordValidated = x"
      />
      <n-space justify="space-evenly" size="large">
        <n-button block class="full-width-btn" size="large" @click="signupClick" type="primary"
                  :disabled="inProgress"
        >
          <div style="display: inline-block">
            <span v-if="!inProgress">注册</span>
            <SpinIndicator v-else/>
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
