<template>
    <div class="login">
      <h2>Login</h2>
      <form @submit.prevent="handleLogin">
        <input type="text" v-model="username" placeholder="Username" required />
        <input type="password" v-model="password" placeholder="Password" required />
        <button type="submit">Login</button>
      </form>
      <NuxtLink to="/register">Register</NuxtLink>
    </div>
  </template>
  
  <script>
  export default {
    data() {
      return {
        username: '',
        password: ''
      };
    },
    methods: {
      handleLogin() {
        // Handle login logic here
        console.log('Logging in with', this.username, this.password);
        // call the api to login
        fetch('http://localhost:8080/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                username: this.username,
                password: this.password
            })  
        })
        .then(response => response.json())
        .then(data => {
            console.log('Login successful', data);
            // store the token in local storage
            localStorage.setItem('token', data.token);
            // redirect to the home page
            this.$router.push('/');
        })

    }
  };
  </script>
  
  <style scoped>
  .login {
    /* Add your styles here */
  }
  </style>