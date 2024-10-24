<script setup lang="ts">
import {ref} from "vue";
import {CHECK_DIGITS} from "../main.ts";

let reason = ref('');
let mobileNumber = ref('');

let emit = defineEmits(['cancel']);
</script>

<template>
  <n-form size="large"
          label-placement="left"
          label-align="left"
          label-width="auto"
  >
    <n-form-item label="申请说明">
      <n-input type="textarea" rows="7" v-model:value="reason"/>
    </n-form-item>
    <n-form-item label="联系电话" required>
      <n-input :allow-input="CHECK_DIGITS" v-model:value="mobileNumber"/>
    </n-form-item>
    <n-space justify="end">
      <n-button type="primary" secondary @click="emit('cancel')">取消</n-button>
      <n-button type="primary">提交</n-button>
    </n-space>
  </n-form>
</template>

<style scoped lang="scss">

</style>
