<script setup lang="ts">
import { invoke } from '@tauri-apps/api'
import { dataDir, join, resolveResource } from '@tauri-apps/api/path';
import { computed } from '@vue/reactivity';
import { onBeforeMount, onBeforeUnmount, onMounted, ref, watch } from 'vue';
import ModCard from './components/ModCard.vue';
import PathInput from './components/PathInput.vue';
import { State, useStore } from './store';
import { resourceDir } from '@tauri-apps/api/path';
// const resourceDirPath = ref("")
// const resource = ref("")

const store = useStore()

const modDir = computed(() => store.state.modDir)

// const modDir = ref(".")
const modFileNameList = ref([])


onBeforeMount(async () => {
  // resource.value = await resolveResource("state.json")
  // resourceDirPath.value = await resourceDir()
  const stateJsonFilePath = await resolveResource("state.json")
  // console.log(stateJsonFilePath)
  if (await invoke('is_dir_exist', { path: stateJsonFilePath })) {
    const stateJson = <string>await invoke('get_json', { path: stateJsonFilePath })
    const state = JSON.parse(stateJson)
    store.commit('updateState', state)
    // console.log(state)
  } else {
    await invoke('save_json', {
      path: stateJsonFilePath,
      json: JSON.stringify(store.state)
    })
  }
})

onMounted(getModList)

watch(modDir, () => getModList())

function onUpdateModDir(newModDir: string) {
  store.commit('updateModDir', newModDir)
}

async function getModList() {
  if (modDir.value === "") return
  modFileNameList.value = await invoke('get_mod_file_name_list', { path: modDir.value })
  // console.log(modPathList.value)
}

</script>

<template>
  <div class="flex flex-1 w-full">
    <!--侧边栏-->
    <!-- <div class="flex flex-col bg-white border-r w-1/4 p-2">
      <h2>Managers</h2>
      <ul class="menu bg-base-100">
        <li><a>Item 1</a></li>
        <li class="bordered"><a>Item 2</a></li>
        <li class="text-gray-400"><a><i class="ri-add-line" /> Add a manager</a></li>
      </ul>
    </div> -->

    <div class="flex flex-col flex-1 bg-white p-2">
      <!-- Mod folder -->
      <PathInput :model-value="modDir" @update:model-value="onUpdateModDir" />
      <!-- {{modDir}} -->


      <!--Header-->
      <!-- <div class="flex flex-col rounded-md border w-full p-2 mb-2">
        <div class="flex">
          <span>Name:</span>
          <span>xxx</span>
        </div>
        <div class="flex">
          <span>Path:</span>
          <span>xxx/xxx/xxx</span>
        </div>
      </div> -->

      <!--Tabs-->
      <!-- <div class="tabs border-b">
        <a class="tab tab-bordered">Versions</a> 
        <a class="tab tab-bordered tab-active">Mods</a> 
        <a class="tab tab-bordered">...</a>
      </div> -->

      <!--Content-->
      <div class="flex-1 w-full p-1 overflow-x-hidden overflow-y-auto">
        <!-- <HelloWorld msg="heihei" /> -->
        <ModCard v-for="modFileName in modFileNameList" :modFileName="modFileName" />
        <!-- <ModCard modPath="testModPath1"/>
        <ModCard modPath="testModPath2"/>
        <ModCard modPath="testModPath3"/> -->
      </div>
    </div>
  </div>
  <!-- <div>
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo" alt="Vite logo" />
    </a>
    <a href="https://vuejs.org/" target="_blank">
      <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
    </a>
  </div>
  <HelloWorld msg="Vite + Vue" /> -->

</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
