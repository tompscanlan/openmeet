<template>
    <div>
      <h1>Register</h1>
      <form @submit.prevent="register">
        <input v-model="user.name" type="text" placeholder="Name" required>
        <input v-model="user.email" type="email" placeholder="Email" required>
        
        <input v-model="user.password" type="password" placeholder="Password" required>
        <button type="submit">Register</button>
      </form>
    </div>
  </template>
  
  <script setup>
  const user = ref({
    email: '',
    password: ''
  })
  
  const register = async () => {
  try {
    const { data, error } = await useFetch('/api/register', {
      method: 'POST',
      body: user.value
    })
    if (error.value) throw error.value
    // Handle successful registration (e.g., redirect to login page)
    navigateTo('/login')
  } catch (err) {
    // Handle registration error
    console.error('Registration failed:', err)
  }
}
  </script>