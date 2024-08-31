<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';
import LoginPage from './pages/Login.vue';
import AuthenticatedPage from './pages/AuthenticatedPage.vue';
import UnauthenticatedPage from './pages/UnauthenticatedPage.vue';
import LogoutPage from './pages/Logout.vue';
import { computed } from 'vue';

const isAuthenticated = ref(false);
const currentPage = ref('unauthenticated');

let routes = {
  '/': UnauthenticatedPage ,
  '/login': LoginPage ,
  '/authenticated': AuthenticatedPage ,
  '/logout': LogoutPage ,
}

const currentPath = ref(window.location.hash)

window.addEventListener('hashchange', () => {
  currentPath.value = window.location.hash
})

const currentView = computed(() => {
  let newpath:string = currentPath.value.slice(1) || '/'
  return routes[newpath as keyof typeof routes] || routes['/']
})

const login = async (username: string, password: string) => {
  try {
    const result = await invoke('login_user', { username, password });
    if (result === true) {
      isAuthenticated.value = true;
      currentPage.value = 'authenticated';
    } else {
      throw new Error('Invalid credentials');
    }
  } catch (error) {
    console.error('Login error:', error);
    alert('Login failed. Please check your credentials.');
  }
};

const logout = async () => {
  try {
    await invoke('logout_user');
    isAuthenticated.value = false;
    currentPage.value = 'logout';
  } catch (error) {
    console.error('Logout error:', error);
  }
};

const navigateTo = (page: string) => {
  if (page === 'authenticated' && !isAuthenticated.value) {
    alert('Please log in first.');
    currentPage.value = 'login';
  } else {
    currentPage.value = page;
  }
};

onMounted(async () => {
  try {
    const authStatus = await invoke('check_auth_status');
    isAuthenticated.value = authStatus === true;
  } catch (error) {
    console.error('Auth check error:', error);
    isAuthenticated.value = false;
  }
});
</script>

<template>
  <div class="container">
    <h1>OpenMeet</h1>
    <nav>
      <a href="#/">Home</a>
      <a href="#/login" v-if="!isAuthenticated">Login</a>
      <a href="#/logout" v-if="isAuthenticated">Logout</a>
      <a href="#/authenticated" v-if="isAuthenticated">Authenticated Page</a>
      <a href="#/unauthenticated" >unauth page</a>

    </nav>

    <component :is="currentView" />
  </div>
</template>

<style scoped>
.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

nav {
  margin-bottom: 20px;
  margin-left: 1pt;
  margin-right: 1pt;
}

nav a {
  margin: 1pt;
}

nav button {
  margin: 0 5px;
}
  form {
     display: flex;
     flex-direction: column;
     gap: 1em;
     margin-bottom: 2em;
   }

   input,
   button {
     border-radius: 8px;
     border: 1px solid transparent;
     padding: 0.6em 1.2em;
     font-size: 1em;
     font-weight: 500;
     font-family: inherit;
     color: #0f0f0f;
     background-color: #ffffff;
     transition: border-color 0.25s;
     box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
   }

   button {
     cursor: pointer;
   }

   button:hover {
     border-color: #396cd8;
   }

   button:active {
     border-color: #396cd8;
     background-color: #e8e8e8;
   }

   input,
   button {
     outline: none;
   }

   .error-message {
     color: red;
     margin-bottom: 1em;
   }
   </style>