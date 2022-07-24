<script setup lang="ts">
import { ref } from 'vue';

const checked = ref(false)

</script>

<template>
  <div class="w-full rounded-md border flex p-2 items-center m-1">
    <div class="flex flex-col items-center">
      <h2>ModName</h2>
      <div class="w-16 h-16 bg-gray-200 rounded text-center">
        Icon
      </div>
    </div>

    <div class="flex-1"></div>

    <div class="dropdown dropdown-end m-1">
      <div class="indicator">
        <span class="indicator-item badge badge-success"></span> 
        <div class="tooltip" data-tip="New Version">
          <label tabindex="0" class="btn">Version</label>
        </div>
      </div>
      <ul tabindex="0" class="dropdown-content menu p-2 shadow bg-base-100 rounded-box w-52">
        <li><a>Item 1</a></li>
        <li><a>Item 2</a></li>
      </ul>
    </div>

    <input type="checkbox" class="checkbox" v-model="checked"/>

  </div>
</template>

<style scoped>
</style>