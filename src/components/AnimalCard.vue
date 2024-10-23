<script setup lang="ts">
import {ref} from "vue";
import AdoptionRequest from "./AdoptionRequest.vue";
import {useRouter} from "vue-router";

let username = ref('用户名');

let emit = defineEmits(['imageClick', 'adoptionClick', 'userProfileClick'])
let router = useRouter();

let showAdoptionModal = ref(false);

function adoptionClick() {
  emit('adoptionClick');
  showAdoptionModal.value = true;
}
</script>

<template>
  <n-modal v-model:show="showAdoptionModal">
    <n-card
        style="width: 600px"
        title="申请领养"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <AdoptionRequest @cancel="showAdoptionModal = false"/>
    </n-card>
  </n-modal>

  <div id="parent">
    <div class="center-parent">
      <img src="/2.jpg" alt="image" style="max-width: 100%"
           @click="router.push('/animal/1'); emit('imageClick')"
           id="img"
      />
      <n-h2 class="label">流浪狗</n-h2>
    </div>
    <div style="padding: 5px">
      <n-text style="word-wrap: break-word">
        路边发现的流浪狗，，，，，，，，，，，，，，，，，，，，
      </n-text>
    </div>
    <div id="bottom-flex">
      <div id="username-div" @click="emit('userProfileClick')">
        <n-avatar src="/avatar-demo.jpg" round/>
        <span id="username-text">{{ username }}</span>
      </div>
      <n-button @click="adoptionClick">申请领养</n-button>
    </div>
  </div>
</template>

<style scoped lang="scss">
.label {
  margin: 0;
  padding: 0;
}

.center-parent {
  text-align: center;
}

#parent {
  display: inline-block;
  padding: .8em;
  width: 20em;
  background-color: white;
  transition: margin-top 0.2s ease-in-out;
}

#parent:hover {
  margin-top: -4px;
  transition: 0.2s ease-in-out;
  box-shadow: rgba(255, 255, 255, 0.1) 0 1px 1px 0 inset, rgba(50, 50, 93, 0.25) 0px 50px 100px -20px, rgba(0, 0, 0, 0.3) 0px 30px 60px -30px;
}

#username-div {
  padding-top: .5em;
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

#bottom-flex {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

#img {
  cursor: pointer;
}

</style>
