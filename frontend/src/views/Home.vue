<template>
  <div>
    <h1>Welcome, {{ user.name }}!</h1>
    <p>Email: {{ user.email }}</p>
    <p>Your API token is: {{ user.api_key }}</p>
    <button @click="handleRefreshApiKey">Refresh API Key</button>
    <br/>
    <br/>
    <router-link v-if="user.permission == 1" to="/admin">Admin Page</router-link>
    <br/>
    <br/>
  
    <button @click="handleLogout">Logout</button>
  </div>
</template>

<script>
export default {
  computed: {
    user() {
      return this.$route.meta.user;
    },
  },
  methods: {
    handleLogout() {
      localStorage.removeItem('authToken');
      this.$router.push('/login');
    },
    handleRefreshApiKey() {
      const authToken = localStorage.getItem('authToken');
      fetch('http://localhost:8080/dashboard/users/refresh_api_key', {
        method: 'POST',
        headers: {
          Authorization: `${authToken}`,
        },
      })
        .then((response) => {
          if (!response.ok) {
            throw new Error('Failed to refresh API key');
          }
          return response.json();
        })
        .then((data) => {
          this.$set(this.$route.meta.user, 'api_key', data.api_key);
        })
        .catch((error) => {
          alert(error.message);
        });
    }
  }
};
</script>

<style>
</style>