<script setup lang="ts">
import {computed, ref} from "vue";
import AdoptionRequest from "./AdoptionRequest.vue";
import {useRouter} from "vue-router";

let emit = defineEmits(['imageClick', 'adoptionClick', 'userProfileClick'])
let router = useRouter();

let showAdoptionModal = ref(false);

function adoptionClick() {
  emit('adoptionClick');
  showAdoptionModal.value = true;
}

const props = defineProps<{
  coverImage?: string,
  name?: string,
  description?: string,
  userAvatarImage?: string,
  username?: string,
  loading?: boolean,
}>();

let loading = computed(() => props.loading);
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
      <img :src="props.coverImage" alt="image" style="max-width: 100%"
           @click="router.push('/animal/1'); emit('imageClick')"
           id="img"
           v-if="!loading"
      />
      <n-skeleton
          v-else
          height="150px"
      />
      <n-h2 class="label">
        <span v-if="!loading">{{ props.name }}</span>
        <n-skeleton width="50%" style="margin-top: .1em" v-else/>
      </n-h2>

    </div>
    <div style="padding: 5px">
      <n-text style="word-wrap: break-word">
        <span v-if="!loading">{{ props.description }}</span>
        <n-skeleton v-else/>
      </n-text>
    </div>
    <div id="bottom-flex">
      <div id="username-div" @click="emit('userProfileClick')">
        <n-avatar :src="props.userAvatarImage" round v-if="!loading"/>
        <n-avatar src="" round v-else/>

        <span id="username-text" v-if="!loading">{{ props.username }}</span>
      </div>
      <n-button @click="adoptionClick" :disabled="loading">申请领养</n-button>
    </div>
  </div>
</template>

<style scoped lang="scss">
.label {
  margin: 0;
  padding: 0;
  display: flex;
  justify-content: center;
}

.center-parent {
  text-align: center;
}

#parent {
  display: inline-block;
  padding: .8em;
  width: 25em;
  background-color: white;
  transition: margin-top 0.2s ease-in-out;
  border-radius: 5px;
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
