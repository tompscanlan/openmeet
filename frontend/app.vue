<template>
  <div class="container">
    <nav>
      <NuxtLink to="/">Home</NuxtLink>
      <!-- <NuxtLink to="/login">Login</NuxtLink> -->
      <NuxtLink to="/register">Register</NuxtLink>
    </nav>
    <NuxtPage />
 
  </div>
</template>

<script>
import LoginComponent from './components/LoginComponent.vue'

export default {
  name: 'HomePage',
  components: {
    LoginComponent
  }
}
</script>

<style scoped>
nav {
  display: flex;
  justify-content: space-between;
  align-items: right;
  padding: 1rem;
  background-color: #f0f0f0;
}
</style>