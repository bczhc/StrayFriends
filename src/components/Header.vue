<script setup lang="ts">
import {Component, h, ref} from "vue";
import {NIcon} from "naive-ui";
import {LogOutOutline, PersonCircleOutline} from "@vicons/ionicons5";
import {useRouter} from "vue-router";
import {JWT_CLEAR} from "../jwt.ts";
import UserInfo from "./UserInfo.vue";

let router = useRouter();
let showUserInfoModal = ref(false);

function renderIcon(icon: Component) {
  return () => {
    return h(NIcon, null, {
      default: () => h(icon)
    });
  };
}

type DropdownKeys = 'my info' | 'logout';

let dropdownOptions = [
  {
    label: '我的信息',
    icon: renderIcon(PersonCircleOutline),
    key: 'my info'
  },
  {
    label: '退出登录',
    icon: renderIcon(LogOutOutline),
    key: 'logout'
  }
];

function onDropdownSelected(key: DropdownKeys) {
  switch (key) {
    case "my info":
      showUserInfoModal.value = true;
      break;
    case "logout":
      JWT_CLEAR();
      router.push({path: '/', query: {type: 'logout'}})
      break;
  }
}
</script>

<template>
  <n-modal v-model:show="showUserInfoModal">
    <n-card
        style="width: 600px"
        title="用户资料"
        :bordered="false"
        size="huge"
        role="dialog"
        aria-modal="true"
    >
      <UserInfo/>
    </n-card>
  </n-modal>

  <n-page-header
      class="header"
  >
    <template #title>
      <div style="position: absolute">
        <span class="ver-center" id="header-title">主页</span>
        <img src="/icon.png" alt="" class="icon-img ver-center"
             id="app-icon"
             @click="router.push('/home')">
      </div>
    </template>
    <template #extra>
      <n-dropdown :options="dropdownOptions" @select="onDropdownSelected">
        <n-avatar round size="medium" src="/avatar-demo.jpg"></n-avatar>
      </n-dropdown>
    </template>
    <template #subtitle>
      <div id="subtitle">
        <div>
          <span>流浪动物救助站</span>
          <div id="navigations">
            <n-button text tag="a" class="nav-button" @click="router.push('/home')"><span>发布</span></n-button>
            <n-button text tag="a" class="nav-button" @click="router.push('/square')"><span>广场</span></n-button>
          </div>
        </div>
      </div>
    </template>
  </n-page-header>
  <n-divider style="margin: 0"/>
</template>

<style scoped lang="scss">
.icon-img {
  filter: invert(33%) sepia(82%) saturate(1386%) hue-rotate(350deg) brightness(107%) contrast(91%);
  left: 2.5em;
  width: 4em;
}

.ver-center {
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
}

.header {
  padding: 1em 2em;
}

#header-title {
  width: 2em;
  display: none;
}

#subtitle {
  position: relative;
  left: 8em;
  color: #616161;
}

#subtitle > div {
  display: inline-flex;
  align-items: center;
}

#navigations {
  margin-left: 3em;
  display: inline-flex;
  gap: 3em;
}

#app-icon {
  cursor: pointer;
}
</style>
