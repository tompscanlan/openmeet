<template>
  <div class="login">
    <h2>Login</h2>
    <form @submit.prevent="handleLogin">
      <input type="email" v-model="email" placeholder="Email" required />
      <input
        type="password"
        v-model="password"
        placeholder="Password"
        required
      />
      <button type="submit">Login</button>
    </form>
    <p v-if="error" class="error">{{ error }}</p>
    <NuxtLink to="/register">Register</NuxtLink>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { useRouter } from "vue-router";

const email = ref("");
const password = ref("");
const error = ref("");
const router = useRouter();

const postLogin = async () => {
  try {
    const response = await fetch("http://localhost:8000/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({
        email: email.value,
        password: password.value,
      }),
    });

    const data = await response.json();

    if (!response.ok) {
      throw new Error(data.message || "Login failed");
    }

    if (data.token) {
      return data.token;
    } else {
      throw new Error("No token received");
    }

  } catch (err) {
    console.error("Login failed", err);
    error.value = err.message + error.value;
  }
};

const getUser = async (email) => {
  const token = localStorage.getItem("token");
  const userResponse = await fetch(`http://localhost:8000/whoami/${email}`, {
    method: "GET",
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${token}`,
    },
  });
  const userData = await userResponse.json();
  return userData;
}

const handleLogin = async () => {
  localStorage.setItem("email", email.value);
  try {
    const token = await postLogin();
    
    if (token) {
      localStorage.setItem("token", token);
    
      const user = await getUser(email.value);
      localStorage.setItem("user", JSON.stringify(user));
      console.log("user data in local storage:", user);

      router.push("/");
    } else {
      throw new Error("No token received");
    }
  } catch (err) {
    console.error("Login failed", err);
    error.value = err.message + error.value;
  }



};
</script>
