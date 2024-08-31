<script setup lang="ts">
import { ref } from 'vue';

const username = ref('');
const password = ref('');

const emit = defineEmits(['login']);

const handleSubmit = () => {
  emit('login', username.value, password.value);
};
</script>

<template>
  <div class="login-page">
    <h2>Login</h2>
    <form @submit.prevent="handleSubmit">
      <input v-model="username" type="text" placeholder="Username" required>
      <input v-model="password" type="password" placeholder="Password" required>
      <button type="submit">Login</button>
    </form>
  </div>
</template>

<style scoped>
.login-page {
  display: flex;
  flex-direction: column;
  align-items: center;
}

form {
  display: flex;
  flex-direction: column;
  gap: 1em;
  width: 300px;
}
</style>