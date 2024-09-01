<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

// invoke get users
const users = ref([]);

invoke("get_users").then((data) => {
  users.value = data as any[];
});

</script>

<template>
  <div class="container">
    <h1>Users</h1>
    <ul>
      <li v-for="user in users" :key="user.id">{{ user.name }}</li>
    </ul>
  </div>
</template>
