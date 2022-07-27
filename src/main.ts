import { createApp } from 'vue'
import App from './App.vue'
import './style.css'
import 'remixicon/fonts/remixicon.css'
import { store, key } from './store'

createApp(App).use(store, key).mount('#app')
