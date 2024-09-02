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

export default {
  name: 'HomePage',
  components: {
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