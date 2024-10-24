<script setup lang="ts">
import {ref} from "vue";
import {CHECK_DIGITS, confirmApiRequest, messageError} from "../main.ts";
import {useDialog, useMessage} from 'naive-ui';
import {apiPost} from "../api.ts";

let details = ref('');
let mobileNumber = ref('');

let emit = defineEmits(['cancel', 'success']);
let props = defineProps<{
  postId?: number
}>();

if (!props.postId) {
  throw "`postId` should be present"
}

const dialog = useDialog();
const message = useMessage();

async function submit() {
  return await apiPost('/api/adoption', {
    postId: props.postId,
    details: details.value,
    mobileNumber: mobileNumber.value,
  });
}

function submitClick() {
  confirmApiRequest(
      dialog,
      '提交申请',
      '确认提交吗？',
      finish => {
        submit()
            .then(_r => {
              emit('success');
              message.success('已提交申请');
            })
            .catch(e => messageError(e, message))
            .finally(() => {
              finish();
            })
      }
  )
}
</script>

<template>
  <n-form size="large"
          label-placement="left"
          label-align="left"
          label-width="auto"
  >
    <n-form-item label="申请说明">
      <n-input type="textarea" rows="7" v-model:value="details"/>
    </n-form-item>
    <n-form-item label="联系电话" required>
      <n-input :allow-input="CHECK_DIGITS" v-model:value="mobileNumber"/>
    </n-form-item>
    <n-space justify="end">
      <n-button type="primary" secondary @click="emit('cancel')">取消</n-button>
      <n-button type="primary" @click="submitClick">提交</n-button>
    </n-space>
  </n-form>
</template>

<style scoped lang="scss">

</style>
