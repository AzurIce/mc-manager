import { invoke } from '@tauri-apps/api'
import { join, resolveResource } from '@tauri-apps/api/path'
import { InjectionKey } from 'vue'
import { createStore, Store, useStore as baseUseStore } from 'vuex'

export interface State {
  modDir: string
}

async function saveState() {
  invoke('save_json', {
    path: await resolveResource("state.json"),
    json: JSON.stringify(store.state)
  })
}

export const key: InjectionKey<Store<State>> = Symbol()

export const store = createStore<State>({
  state: {
    modDir: "./"
  },
  mutations: {
    updateModDir (state, newModDir: string) {
      state.modDir = newModDir
      saveState()
    },
    updateState (state, newState: State) {
      console.log(newState)
      state.modDir = newState.modDir
    }
  }
})

export function useStore() {
  return baseUseStore(key)
}